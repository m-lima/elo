use super::{super::model, *};

#[sqlx::test]
async fn pending_only(pool: sqlx::sqlite::SqlitePool) {
    let mut handler = init!(pool).2;

    handler
        .call(model::Request::Invite(model::request::Invite::Accept))
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(model::Request::Invite(model::request::Invite::Reject))
        .await
        .err(model::Error::Forbidden)
        .unwrap();
}

#[sqlx::test]
async fn regular_only(pool: sqlx::sqlite::SqlitePool) {
    let (_, store, mut handler) = init!(pool);
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    let mut handler = framework::Handler::pending(&invited.email, &store)
        .await
        .unwrap();

    handler
        .call(model::Request::Player(model::request::Player::List))
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(model::Request::Player(model::request::Player::Rename(
            String::new(),
        )))
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(model::Request::Game(model::request::Game::List))
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(model::Request::Game(model::request::Game::Register {
            opponent: 0,
            score: 0,
            opponent_score: 0,
            challenge: false,
        }))
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(model::Request::Invite(model::request::Invite::List))
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(model::Request::Invite(model::request::Invite::Cancel(0)))
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(model::Request::Invite(model::request::Invite::Player {
            name: String::new(),
            email: String::new(),
        }))
        .await
        .err(model::Error::Forbidden)
        .unwrap();
}

#[sqlx::test]
async fn id(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    handler
        .call(model::Request::Player(model::request::Player::Id))
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
        .call(model::Request::Player(model::request::Player::Id))
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
