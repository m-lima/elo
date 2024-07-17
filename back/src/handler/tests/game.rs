use super::{super::model, *};

#[sqlx::test]
async fn list(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    handler
        .call(model::Request::Game(model::request::Game::List))
        .await
        .ok(model::Response::Games(Vec::new()))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let model::Push::Game(model::push::Game::Registered(game, _, _)) = handler
        .call(model::Request::Game(model::request::Game::Register {
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: false,
        }))
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

    handler
        .call(model::Request::Game(model::request::Game::List))
        .await
        .ok(model::Response::Games(vec![types::GameTuple::from(game)]))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn register(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let model::Push::Game(model::push::Game::Registered(game, one, two)) = handler
        .call(model::Request::Game(model::request::Game::Register {
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: false,
        }))
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

    assert_eq!(game.player_one, player.id);
    assert_eq!(game.player_two, accepted.id);
    assert_eq!(game.score_one, 11);
    assert_eq!(game.score_two, 0);
    assert!((game.rating_one - player.rating).abs() < f64::EPSILON);
    assert!((game.rating_two - accepted.rating).abs() < f64::EPSILON);
    assert!(!game.challenge);

    let expected_scores = skillratings::elo::elo(
        &skillratings::elo::EloRating {
            rating: player.rating,
        },
        &skillratings::elo::EloRating {
            rating: accepted.rating,
        },
        &skillratings::Outcomes::WIN,
        &skillratings::elo::EloConfig::new(),
    );

    let player = types::Player {
        rating: expected_scores.0.rating,
        ..player
    };
    let accepted = types::Player {
        rating: expected_scores.1.rating,
        ..accepted
    };

    assert_eq!(one, player);
    assert_eq!(two, accepted);

    handler
        .call(model::Request::Game(model::request::Game::List))
        .await
        .ok(model::Response::Games(vec![types::GameTuple::from(game)]))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    handler
        .call(model::Request::Player(model::request::Player::List))
        .await
        .ok(model::Response::Players(
            [player, accepted]
                .map(types::PlayerTuple::from)
                .into_iter()
                .collect(),
        ))
        .unwrap();
}

#[sqlx::test]
async fn register_not_found(pool: sqlx::sqlite::SqlitePool) {
    let mut handler = init!(pool).2;

    handler
        .call(model::Request::Game(model::request::Game::Register {
            opponent: 27,
            score: 11,
            opponent_score: 0,
            challenge: false,
        }))
        .await
        .err(model::Error::Store(store::Error::NotFound))
        .unwrap();
}

#[sqlx::test]
async fn register_same_player(pool: sqlx::sqlite::SqlitePool) {
    let (player, _, mut handler) = init!(pool);

    handler
        .call(model::Request::Game(model::request::Game::Register {
            opponent: player.id,
            score: 11,
            opponent_score: 0,
            challenge: false,
        }))
        .await
        .err(model::Error::Store(store::Error::InvalidValue(
            "Players cannot be equal",
        )))
        .unwrap();
}

#[sqlx::test]
async fn register_good_score(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    for score in 0..=10 {
        handler
            .call(model::Request::Game(model::request::Game::Register {
                opponent: accepted.id,
                score: 11,
                opponent_score: score,
                challenge: false,
            }))
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap();

        handler
            .call(model::Request::Game(model::request::Game::Register {
                opponent: accepted.id,
                score,
                opponent_score: 11,
                challenge: false,
            }))
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap();
    }

    handler
        .call(model::Request::Game(model::request::Game::Register {
            opponent: accepted.id,
            score: 12,
            opponent_score: 10,
            challenge: false,
        }))
        .await
        .done()
        .unwrap()
        .none()
        .unwrap()
        .some()
        .unwrap();

    handler
        .call(model::Request::Game(model::request::Game::Register {
            opponent: accepted.id,
            score: 10,
            opponent_score: 12,
            challenge: false,
        }))
        .await
        .done()
        .unwrap()
        .none()
        .unwrap()
        .some()
        .unwrap();
}

#[sqlx::test]
async fn register_bad_score(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    for (score, opponent_score, challenge) in (0..15)
        .flat_map(|one| (0..15).map(move |two| (one, two)))
        .filter(|&(one, two)| {
            one > 12
                || two > 12
                || (one == 11 && two > 10)
                || (two == 11 && one > 10)
                || (one == 12 && two != 10)
                || (two == 12 && one != 10)
        })
        .flat_map(|(one, two)| [true, false].map(|c| (one, two, c)))
    {
        handler
            .call(model::Request::Game(model::request::Game::Register {
                opponent: accepted.id,
                score,
                opponent_score,
                challenge,
            }))
            .await
            .err(model::Error::Store(store::Error::InvalidValue(
                if score == opponent_score {
                    "Scores cannot be equal"
                } else if score > 12 || opponent_score > 12 {
                    "Games cannot have a score larger than 12"
                } else if score < 11 && opponent_score < 11 {
                    "Games must have a winner with at least 11 points"
                } else if score == 12 || opponent_score == 12 {
                    "Tie breaks require a 12x10 score"
                } else {
                    "There can only be one winner"
                },
            )))
            .unwrap();
    }
}

#[ignore]
#[sqlx::test]
async fn register_challenge_daily_limit(pool: sqlx::sqlite::SqlitePool) {
    // TODO: Test this once we can set dates and remove test from store::store::games::tests
    panic!("{pool:?}")
}

#[sqlx::test]
async fn forbidden(pool: sqlx::sqlite::SqlitePool) {
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
