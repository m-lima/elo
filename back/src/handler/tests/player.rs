use super::{super::model, *};

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;

#[sqlx::test]
async fn id(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

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
async fn list(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);
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
async fn ratings(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);
    handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();
    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    // Check with no games
    handler
        .call(
            model::Request::Player(model::request::Player::Ratings),
            false,
        )
        .await
        .ok(model::Response::Ratings(Vec::new()))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    // Check with a game a long time ago
    handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: types::Millis::from(0),
            }),
            true,
        )
        .await
        .done()
        .unwrap()
        .none()
        .unwrap()
        .some()
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Ratings),
            false,
        )
        .await
        .ok(model::Response::Ratings(vec![
            types::RatingTuple(player.id, 1000.0, types::Millis::from(0)),
            types::RatingTuple(accepted.id, 1000.0, types::Millis::from(0)),
        ]))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    // Check with a game just now
    let now = types::Millis::now();

    handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: now,
            }),
            true,
        )
        .await
        .done()
        .unwrap()
        .none()
        .unwrap()
        .some()
        .unwrap();

    handler
        .call(
            model::Request::Player(model::request::Player::Ratings),
            false,
        )
        .await
        .map_ok(
            |response| {
                let model::Response::Ratings(mut list) = response else {
                    panic!()
                };
                for t in &mut list {
                    t.1 = t.1.round();
                }
                list
            },
            vec![
                types::RatingTuple(player.id, DEFAULT_RATING + default_rating_delta(), now),
                types::RatingTuple(accepted.id, DEFAULT_RATING - default_rating_delta(), now),
            ],
        )
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn rename(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);
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
async fn invalid_input(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let mut handler = init!(pool, conn).2;

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
async fn repeated_input(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);
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
async fn forbidden(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (_, store, mut handler, _) = init!(pool, conn);
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
