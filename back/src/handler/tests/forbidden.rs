use super::*;

#[sqlx::test]
async fn pending_only(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Accept))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Reject))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }
}

#[sqlx::test]
async fn regular_only(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;
    let mut handler = RichHandler::pending(&invited.email, &store).await;

    match handler
        .call_err(model::Request::Player(model::request::Player::List))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Player(model::request::Player::Rename(
            String::new(),
        )))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Game(model::request::Game::List))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Game(model::request::Game::Register {
            opponent: 0,
            score: 0,
            opponent_score: 0,
            challenge: false,
        }))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::List))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Cancel(0)))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: String::new(),
            email: String::new(),
        }))
        .await
    {
        model::Error::Forbidden => {}
        e => panic!("Unexpected error: {e:?}"),
    }
}

#[sqlx::test]
async fn id(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    match handler
        .call_ok(model::Request::Player(model::request::Player::Id))
        .await
    {
        model::Response::User { id, pending } => {
            assert_eq!(id, player.id);
            assert_eq!(pending, None);
        }
        e => panic!("Unexpected response: {e:?}"),
    }

    handler.check_no_message();

    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;

    let mut handler = RichHandler::pending(&invited.email, &store).await;

    match handler
        .call_ok(model::Request::Player(model::request::Player::Id))
        .await
    {
        model::Response::User { id, pending } => {
            assert_eq!(id, invited.id);
            assert_eq!(pending, Some(true));
        }
        e => panic!("Unexpected response: {e:?}"),
    }
}
