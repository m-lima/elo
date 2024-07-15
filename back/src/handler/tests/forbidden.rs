use super::*;

#[sqlx::test]
async fn invite(pool: sqlx::sqlite::SqlitePool) {
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
async fn invited(pool: sqlx::sqlite::SqlitePool) {
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
