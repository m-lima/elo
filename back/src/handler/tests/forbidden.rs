use super::*;

#[sqlx::test]
async fn pending_only(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await.unwrap();
    let mut handler = framework::Handler::new(&player.email, &store).await;

    handler
        .call_err(
            model::Request::Invite(model::request::Invite::Accept),
            model::Error::Forbidden,
        )
        .await
        .unwrap();

    handler
        .call_err(
            model::Request::Invite(model::request::Invite::Reject),
            model::Error::Forbidden,
        )
        .await
        .unwrap();
}

#[sqlx::test]
async fn regular_only(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;
    let invited = handler
        .invite(INVITED_NAME, INVITED_EMAIL, player.id)
        .await
        .unwrap();
    let mut handler = RichHandler::pending(&invited.email, &store).await;

    handler
        .call_err(
            model::Request::Player(model::request::Player::List),
            model::Error::Forbidden,
        )
        .await
        .unwrap();

    handler
        .call_err(
            model::Request::Player(model::request::Player::Rename(String::new())),
            model::Error::Forbidden,
        )
        .await
        .unwrap();

    handler
        .call_err(
            model::Request::Game(model::request::Game::List),
            model::Error::Forbidden,
        )
        .await
        .unwrap();

    handler
        .call_err(
            model::Request::Game(model::request::Game::Register {
                opponent: 0,
                score: 0,
                opponent_score: 0,
                challenge: false,
            }),
            model::Error::Forbidden,
        )
        .await
        .unwrap();

    handler
        .call_err(
            model::Request::Invite(model::request::Invite::List),
            model::Error::Forbidden,
        )
        .await
        .unwrap();

    handler
        .call_err(
            model::Request::Invite(model::request::Invite::Cancel(0)),
            model::Error::Forbidden,
        )
        .await
        .unwrap();

    handler
        .call_err(
            model::Request::Invite(model::request::Invite::Player {
                name: String::new(),
                email: String::new(),
            }),
            model::Error::Forbidden,
        )
        .await
        .unwrap();
}

#[sqlx::test]
async fn id(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    handler
        .call_ok(
            model::Request::Player(model::request::Player::Id),
            model::Response::User {
                id: player.id,
                pending: None,
            },
        )
        .await;

    handler.check_no_message();

    let invited = handler
        .invite(INVITED_NAME, INVITED_EMAIL, player.id)
        .await
        .unwrap();

    let mut handler = RichHandler::pending(&invited.email, &store).await;

    handler
        .call_ok(
            model::Request::Player(model::request::Player::Id),
            model::Response::User {
                id: invited.id,
                pending: Some(true),
            },
        )
        .await
        .unwrap();
}
