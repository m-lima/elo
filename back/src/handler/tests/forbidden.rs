use super::{super::model, *};

#[sqlx::test]
async fn regular_only(pool: sqlx::sqlite::SqlitePool) {
    let (_, store, mut handler) = init!(pool);
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    let mut handler = framework::Handler::pending(&invited.email, &store)
        .await
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
}
