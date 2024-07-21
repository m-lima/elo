use super::{super::model, *};
use crate::types;

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

    let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
        .call(model::Request::Game(model::request::Game::Register {
            player: player.id,
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: false,
            millis: super::now(),
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

    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, game);

    handler
        .call(model::Request::Game(model::request::Game::List))
        .await
        .ok(model::Response::Games(updates))
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

    let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
        .call(model::Request::Game(model::request::Game::Register {
            player: player.id,
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: false,
            millis: super::now(),
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

    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, game);

    let game = {
        let game = updates.into_iter().next().unwrap();
        types::Game {
            id: game.0,
            player_one: game.1,
            player_two: game.2,
            score_one: game.3,
            score_two: game.4,
            rating_one: game.5,
            rating_two: game.6,
            rating_delta: game.7,
            challenge: game.8,
            deleted: game.9,
            millis: game.10,
            created_ms: game.11,
        }
    };

    let rating_delta = skillratings::elo::elo(
        &skillratings::elo::EloRating::new(),
        &skillratings::elo::EloRating::new(),
        &skillratings::Outcomes::WIN,
        &skillratings::elo::EloConfig::new(),
    )
    .0
    .rating
        - skillratings::elo::EloRating::new().rating;

    assert_eq!(game.player_one, player.id);
    assert_eq!(game.player_two, accepted.id);
    assert_eq!(game.score_one, 11);
    assert_eq!(game.score_two, 0);
    assert!((game.rating_one - skillratings::elo::EloRating::new().rating).abs() <= f64::EPSILON);
    assert!((game.rating_two - skillratings::elo::EloRating::new().rating).abs() <= f64::EPSILON);
    assert!((game.rating_delta - rating_delta).abs() <= f64::EPSILON);
    assert!(!game.challenge);

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
async fn register_to_other_players(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    let accepted_one = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let accepted_two = handler
        .invite_full(&player, &store, INVITED_NAME, INVITED_EMAIL)
        .await
        .unwrap();

    let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
        .call(model::Request::Game(model::request::Game::Register {
            player: accepted_one.id,
            opponent: accepted_two.id,
            score: 11,
            opponent_score: 0,
            challenge: false,
            millis: super::now(),
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

    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, game);

    let game = {
        let game = updates.into_iter().next().unwrap();
        types::Game {
            id: game.0,
            player_one: game.1,
            player_two: game.2,
            score_one: game.3,
            score_two: game.4,
            rating_one: game.5,
            rating_two: game.6,
            rating_delta: game.7,
            challenge: game.8,
            deleted: game.9,
            millis: game.10,
            created_ms: game.11,
        }
    };

    let rating_delta = skillratings::elo::elo(
        &skillratings::elo::EloRating::new(),
        &skillratings::elo::EloRating::new(),
        &skillratings::Outcomes::WIN,
        &skillratings::elo::EloConfig::new(),
    )
    .0
    .rating
        - skillratings::elo::EloRating::new().rating;

    assert_eq!(game.player_one, accepted_one.id);
    assert_eq!(game.player_two, accepted_two.id);
    assert_eq!(game.score_one, 11);
    assert_eq!(game.score_two, 0);
    assert!((game.rating_one - skillratings::elo::EloRating::new().rating).abs() <= f64::EPSILON);
    assert!((game.rating_two - skillratings::elo::EloRating::new().rating).abs() <= f64::EPSILON);
    assert!((game.rating_delta - rating_delta).abs() <= f64::EPSILON);
    assert!(!game.challenge);

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
async fn register_many(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let mut games = Vec::with_capacity(3);
    for _ in 0..3 {
        if let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
            .call(model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: super::now(),
            }))
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap()
        {
            assert_eq!(updates.len(), 1);
            assert_eq!(updates[0].0, game);
            games.push(updates.into_iter().next().unwrap());
        } else {
            panic!()
        }
    }

    handler
        .call(model::Request::Game(model::request::Game::List))
        .await
        .ok(model::Response::Games(games.clone()))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn register_not_found(pool: sqlx::sqlite::SqlitePool) {
    let mut handler = init!(pool).2;

    handler
        .call(model::Request::Game(model::request::Game::Register {
            player: 15,
            opponent: 27,
            score: 11,
            opponent_score: 0,
            challenge: false,
            millis: super::now(),
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
            player: player.id,
            opponent: player.id,
            score: 11,
            opponent_score: 0,
            challenge: false,
            millis: super::now(),
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
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: score,
                challenge: false,
                millis: super::now(),
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
                player: player.id,
                opponent: accepted.id,
                score,
                opponent_score: 11,
                challenge: false,
                millis: super::now(),
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
            player: player.id,
            opponent: accepted.id,
            score: 12,
            opponent_score: 10,
            challenge: false,
            millis: super::now(),
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
            player: player.id,
            opponent: accepted.id,
            score: 10,
            opponent_score: 12,
            challenge: false,
            millis: super::now(),
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
                player: player.id,
                opponent: accepted.id,
                score,
                opponent_score,
                challenge,
                millis: super::now(),
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

#[sqlx::test]
async fn register_challenge_daily_limit(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let millis = 1_704_070_861_000_i64; // 2024-01-01 01:01:01

    let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
        .call(model::Request::Game(model::request::Game::Register {
            player: player.id,
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: true,
            millis: types::Millis::from(millis),
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

    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, game);

    let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
        .call(model::Request::Game(model::request::Game::Register {
            player: player.id,
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: false,
            millis: types::Millis::from(millis + 1),
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

    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, game);

    handler
        .call(model::Request::Game(model::request::Game::Register {
            player: player.id,
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: true,
            millis: types::Millis::from(millis + 1),
        }))
        .await
        .err(model::Error::Store(store::Error::InvalidValue(
            "Players cannot challenge each other more than once a day",
        )))
        .unwrap();

    let millis = 1_704_153_599_999_i64; // 2024-01-01 23:59:59.999

    handler
        .call(model::Request::Game(model::request::Game::Register {
            player: player.id,
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: true,
            millis: types::Millis::from(millis),
        }))
        .await
        .err(model::Error::Store(store::Error::InvalidValue(
            "Players cannot challenge each other more than once a day",
        )))
        .unwrap();

    let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
        .call(model::Request::Game(model::request::Game::Register {
            player: player.id,
            opponent: accepted.id,
            score: 11,
            opponent_score: 0,
            challenge: true,
            millis: types::Millis::from(millis + 1),
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

    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, game);
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
            player: 0,
            opponent: 0,
            score: 0,
            opponent_score: 0,
            challenge: false,
            millis: super::now(),
        }))
        .await
        .err(model::Error::Forbidden)
        .unwrap();
}
