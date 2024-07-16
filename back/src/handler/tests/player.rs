use super::*;

#[sqlx::test]
async fn list(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;
    let accepted = {
        let invited = handler
            .invite("accepted", "accepted@email.com", player.id)
            .await
            .unwrap();

        RichHandler::pending(&invited.email, &store)
            .await
            .accept(&player, &invited)
            .await
            .unwrap()
    };

    handler
        .call_ok(
            model::Request::Player(model::request::Player::List),
            model::Response::Players(
                [player.clone(), accepted.clone()]
                    .map(types::PlayerTuple::from)
                    .into_iter()
                    .collect(),
            ),
        )
        .await;

    handler.check_no_message();
}

#[sqlx::test]
async fn rename(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;
    let accepted = {
        let invited = handler
            .invite("accepted", "accepted@email.com", player.id)
            .await
            .unwrap();

        RichHandler::pending(&invited.email, &store)
            .await
            .accept(&player, &invited)
            .await
            .unwrap()
    };

    handler
        .call_done(model::Request::Player(model::request::Player::Rename(
            String::from("new"),
        )))
        .await;

    // handler.check_no_email();

    handler
        .call_ok(
            model::Request::Player(model::request::Player::List),
            model::Response::Players(
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
            ),
        )
        .await;

    handler.check_no_message();
}
