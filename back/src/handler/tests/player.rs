use super::{super::model, *};

#[sqlx::test]
async fn id(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    handler
        .call(model::Request::Player(model::request::Player::Id), false)
        .await
        .ok(model::Response::User {
            id: player.id,
            pending: None,
        })
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    let mut handler = framework::Handler::pending(&invited.email, &store)
        .await
        .unwrap();
    handler
        .call(model::Request::Player(model::request::Player::Id), false)
        .await
        .ok(model::Response::User {
            id: invited.id,
            pending: Some(true),
        })
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn list(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);
    handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();
    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    handler
        .call(model::Request::Player(model::request::Player::List), false)
        .await
        .ok(model::Response::Players(
            [player.clone(), accepted.clone()]
                .map(types::PlayerTuple::from)
                .into_iter()
                .collect(),
        ))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn rename(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);
    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let model::Push::Player(model::push::Player::Renamed {
        player: rename_player,
        old,
        new,
    }) = handler
        .call(
            model::Request::Player(model::request::Player::Rename(String::from("new"))),
            true,
        )
        .await
        .done()
        .unwrap()
        .none()
        .unwrap()
        .some()
        .unwrap()
    else {
        panic!()
    };

    assert_eq!(rename_player, player.id);
    assert_eq!(old, TESTER_NAME);
    assert_eq!(new, "new");

    handler
        .call(model::Request::Player(model::request::Player::List), false)
        .await
        .ok(model::Response::Players(
            [
                types::Player {
                    name: String::from("new"),
                    ..player.clone()
                },
                accepted.clone(),
            ]
            .map(types::PlayerTuple::from)
            .into_iter()
            .collect(),
        ))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    let model::Push::Player(model::push::Player::Renamed {
        player: rename_player,
        old,
        new,
    }) = handler
        .call(
            model::Request::Player(model::request::Player::Rename(String::from(TESTER_NAME))),
            true,
        )
        .await
        .done()
        .unwrap()
        .none()
        .unwrap()
        .some()
        .unwrap()
    else {
        panic!()
    };

    assert_eq!(rename_player, player.id);
    assert_eq!(old, "new");
    assert_eq!(new, TESTER_NAME);

    handler
        .call(model::Request::Player(model::request::Player::List), false)
        .await
        .ok(model::Response::Players(
            [
                types::Player {
                    name: String::from(TESTER_NAME),
                    ..player.clone()
                },
                accepted.clone(),
            ]
            .map(types::PlayerTuple::from)
            .into_iter()
            .collect(),
        ))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn invalid_input(pool: sqlx::sqlite::SqlitePool) {
    let mut handler = init!(pool).2;

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(String::new())),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::BlankValue("name")))
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(String::from(WHITE_SPACE))),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::BlankValue("name")))
        .unwrap();
}

#[sqlx::test]
async fn repeated_input(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);
    handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();
    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(player.name.clone())),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(String::from(INVITED_NAME))),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(accepted.name.clone())),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(format!(
                "{WHITE_SPACE}{}{WHITE_SPACE}",
                player.name
            ))),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(format!(
                "{WHITE_SPACE}{INVITED_NAME}{WHITE_SPACE}"
            ))),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(format!(
                "{WHITE_SPACE}{}{WHITE_SPACE}",
                accepted.name
            ))),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();
}

#[sqlx::test]
async fn forbidden(pool: sqlx::sqlite::SqlitePool) {
    let (_, store, mut handler) = init!(pool);
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    let mut handler = framework::Handler::pending(&invited.email, &store)
        .await
        .unwrap();

    handler
        .call(model::Request::Player(model::request::Player::List), false)
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Rename(String::new())),
            false,
        )
        .await
        .err(model::Error::Forbidden)
        .unwrap();
}
