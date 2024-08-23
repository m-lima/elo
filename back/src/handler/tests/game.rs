use super::{super::model, *};
use crate::types;

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;

#[sqlx::test]
async fn list(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    handler
        .call(model::Request::Game(model::request::Game::List), false)
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

    let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: super::now(),
            }),
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

    handler
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .ok(model::Response::Games(vec![game.into()]))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn register(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: super::now(),
            }),
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

    assert_eq!(updates.len(), 0);

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
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .ok(model::Response::Games(vec![types::GameTuple::from(game)]))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn register_to_other_players(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    let accepted_one = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let accepted_two = handler
        .invite_full(&player, &store, INVITED_NAME, INVITED_EMAIL)
        .await
        .unwrap();

    let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: accepted_one.id,
                opponent: accepted_two.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: super::now(),
            }),
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

    assert_eq!(updates.len(), 0);

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
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .ok(model::Response::Games(vec![types::GameTuple::from(game)]))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn register_many(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let mut games = Vec::with_capacity(3);
    for _ in 0..3 {
        if let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: player.id,
                    opponent: accepted.id,
                    score: 11,
                    opponent_score: 0,
                    challenge: false,
                    millis: super::now(),
                }),
                true,
            )
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap()
        {
            games.push(game.into());
        } else {
            panic!()
        }
    }

    handler
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .ok(model::Response::Games(games))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn register_not_found(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let mut handler = init!(pool, conn).2;

    handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: 15,
                opponent: 27,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: super::now(),
            }),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::NotFound))
        .unwrap();
}

#[sqlx::test]
async fn register_same_player(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, _, mut handler, _) = init!(pool, conn);

    handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: player.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: super::now(),
            }),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::InvalidValue(
            "Players cannot be equal",
        )))
        .unwrap();
}

#[sqlx::test]
async fn register_good_score(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    for score in 0..=10 {
        handler
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: player.id,
                    opponent: accepted.id,
                    score: 11,
                    opponent_score: score,
                    challenge: false,
                    millis: super::now(),
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
                model::Request::Game(model::request::Game::Register {
                    player: player.id,
                    opponent: accepted.id,
                    score,
                    opponent_score: 11,
                    challenge: false,
                    millis: super::now(),
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
    }

    handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 12,
                opponent_score: 10,
                challenge: false,
                millis: super::now(),
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
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 10,
                opponent_score: 12,
                challenge: false,
                millis: super::now(),
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
}

#[sqlx::test]
async fn register_bad_score(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

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
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: player.id,
                    opponent: accepted.id,
                    score,
                    opponent_score,
                    challenge,
                    millis: super::now(),
                }),
                false,
            )
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
async fn register_challenge_daily_limit(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let millis = 1_704_070_861_000_i64; // 2024-01-01 01:01:01

    let model::Push::Game(model::push::Game::Registered { .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: true,
                millis: types::Millis::from(millis),
            }),
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

    let model::Push::Game(model::push::Game::Registered { .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: types::Millis::from(millis + 1),
            }),
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

    handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: true,
                millis: types::Millis::from(millis + 1),
            }),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::InvalidValue(
            "Players cannot challenge each other more than once a day",
        )))
        .unwrap();

    let millis = 1_704_153_599_999_i64; // 2024-01-01 23:59:59.999

    handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: true,
                millis: types::Millis::from(millis),
            }),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::InvalidValue(
            "Players cannot challenge each other more than once a day",
        )))
        .unwrap();

    let model::Push::Game(model::push::Game::Registered { .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: true,
                millis: types::Millis::from(millis + 1),
            }),
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
}

#[sqlx::test]
async fn edit_challenge_daily_limit(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    let accepted_one = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let accepted_two = handler
        .invite_full(&player, &store, INVITED_NAME, INVITED_EMAIL)
        .await
        .unwrap();

    let millis = 1_704_070_861_000_i64; // 2024-01-01 01:01:01

    let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted_one.id,
                score: 11,
                opponent_score: 0,
                challenge: true,
                millis: types::Millis::from(millis),
            }),
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

    let game_one = game;

    let model::Push::Game(model::push::Game::Registered { .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted_two.id,
                score: 11,
                opponent_score: 0,
                challenge: true,
                millis: types::Millis::from(millis + 1),
            }),
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

    let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted_one.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: types::Millis::from(millis + 2),
            }),
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

    let game_two = game;

    handler
        .call(
            model::Request::Game(model::request::Game::Update(types::Game {
                challenge: true,
                ..game_two.clone()
            })),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::InvalidValue(
            "Players cannot challenge each other more than once a day",
        )))
        .unwrap();

    handler
        .call(
            model::Request::Game(model::request::Game::Update(types::Game {
                player_two: accepted_two.id,
                ..game_one.clone()
            })),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::InvalidValue(
            "Players cannot challenge each other more than once a day",
        )))
        .unwrap();
}

#[sqlx::test]
async fn delete_game(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    // Prepare players
    let (player, store, mut handler, pool) = init!(pool, conn);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    // Create expected output from simply creating
    let mut expected = Vec::with_capacity(5);
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }

        if let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: player.id,
                    opponent: accepted.id,
                    score: 11,
                    opponent_score: i,
                    challenge: false,
                    millis: types::Millis::from(i64::from(i)),
                }),
                true,
            )
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap()
        {
            expected.push(types::Game {
                id: 0,
                created_ms: types::Millis::from(0),
                ..game
            });
        } else {
            panic!()
        }
    }

    // Clear the table
    sqlx::query!("DELETE FROM games")
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(store.games().list().await.unwrap().len(), 0);

    // Create all games
    let mut games = Vec::with_capacity(10);
    for i in 1..=10 {
        if let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: player.id,
                    opponent: accepted.id,
                    score: 11,
                    opponent_score: i,
                    challenge: false,
                    millis: types::Millis::from(i64::from(i)),
                }),
                true,
            )
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap()
        {
            games.push(game);
        } else {
            panic!()
        }
    }

    // Delete the ones to be deleted
    for g in &games {
        if g.score_two % 2 != 0 {
            continue;
        }

        let model::Push::Game(model::push::Game::Updated { .. }) = handler
            .call(
                model::Request::Game(model::request::Game::Update(types::Game {
                    deleted: true,
                    ..g.clone()
                })),
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
            panic!();
        };
    }

    // Check that the output matches the one created without edits
    handler
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .map_ok(
            |r| {
                let model::Response::Games(response) = r else {
                    panic!()
                };
                response
                    .into_iter()
                    .map(|g| types::Game {
                        id: 0,
                        created_ms: types::Millis::from(0),
                        ..types::Game::from(g)
                    })
                    .filter(|g| !g.deleted)
                    .collect()
            },
            expected,
        )
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn random_updates(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    struct ModifiableGame {
        player_one: types::Id,
        player_two: types::Id,
        score_one: u8,
        score_two: u8,
        challenge: bool,
        deleted: bool,
        millis: types::Millis,
    }

    fn make_games(ids: [types::Id; 3], seed: u64) -> Vec<ModifiableGame> {
        use rand::Rng;

        let mut rand: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);

        (0..50)
            .map(|_| {
                let player_one = ids[rand.gen_range(0..3)];
                let player_two = {
                    let mut id = ids[rand.gen_range(0..3)];
                    while id == player_one {
                        id = ids[rand.gen_range(0..3)];
                    }
                    id
                };

                let (score_one, score_two) = {
                    let winner_score = if rand.gen_bool(0.8) { 11 } else { 12 };
                    let loser_score = if winner_score == 12 {
                        10
                    } else {
                        rand.gen_range(0..10)
                    };

                    if rand.gen_bool(0.5) {
                        (winner_score, loser_score)
                    } else {
                        (loser_score, winner_score)
                    }
                };

                let challenge = rand.gen_bool(0.3);
                let deleted = rand.gen_bool(0.2);
                let millis =
                    types::Millis::from(i64::from(rand.gen::<u16>()) * 12 * 60 * 60 * 1000);

                ModifiableGame {
                    player_one,
                    player_two,
                    score_one,
                    score_two,
                    challenge,
                    deleted,
                    millis,
                }
            })
            .collect()
    }

    let seed = rand::random();
    println!("Using seed: {seed}");

    // Prepare players
    let (player_one, store, mut handler, pool) = init!(pool, conn);

    let player_two = handler
        .invite_full(&player_one, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let player_three = handler
        .invite_full(&player_one, &store, INVITED_NAME, INVITED_EMAIL)
        .await
        .unwrap();

    // Create expected output from simply creating
    let games = make_games([player_one.id, player_two.id, player_three.id], seed);
    for game in games {
        if game.deleted {
            continue;
        }

        let model::Push::Game(model::push::Game::Registered { .. }) = handler
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: game.player_one,
                    opponent: game.player_two,
                    score: game.score_one,
                    opponent_score: game.score_two,
                    challenge: game.challenge,
                    millis: game.millis,
                }),
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
    }

    let model::Response::Games(expected) = handler
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .raw()
        .unwrap()
    else {
        panic!()
    };

    let expected = expected
        .into_iter()
        .map(|g| types::Game {
            id: 0,
            created_ms: types::Millis::from(0),
            ..types::Game::from(g)
        })
        .collect::<Vec<_>>();

    // Clear the table
    sqlx::query!("DELETE FROM games")
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(store.games().list().await.unwrap().len(), 0);

    // Create all games
    let targets = make_games([player_one.id, player_two.id, player_three.id], seed);
    let mut games = Vec::with_capacity(targets.len());
    for _ in 0..targets.len() {
        if let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: player_one.id,
                    opponent: player_two.id,
                    score: 11,
                    opponent_score: 0,
                    challenge: false,
                    millis: now(),
                }),
                true,
            )
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap()
        {
            games.push(game);
        } else {
            panic!()
        }
    }

    // Update the games according to the jig
    for (exising, target) in games.into_iter().zip(targets) {
        if let model::Push::Game(model::push::Game::Updated { game, .. }) = handler
            .call(
                model::Request::Game(model::request::Game::Update(types::Game {
                    player_one: target.player_one,
                    player_two: target.player_two,
                    score_one: i64::from(target.score_one),
                    score_two: i64::from(target.score_two),
                    challenge: target.challenge,
                    deleted: target.deleted,
                    millis: target.millis,
                    ..exising
                })),
                true,
            )
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap()
        {
            assert_eq!(game.id, exising.id);
        } else {
            panic!();
        };
    }

    // Check that the output matches the one created without edits
    let model::Response::Games(response) = handler
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .raw()
        .unwrap()
    else {
        panic!()
    };

    let response = response
        .into_iter()
        .map(|g| types::Game {
            id: 0,
            created_ms: types::Millis::from(0),
            ..types::Game::from(g)
        })
        .filter(|g| !g.deleted)
        .collect::<Vec<_>>();

    assert_eq!(response, expected);
}

#[sqlx::test]
async fn creation_time_does_not_matter(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, pool) = init!(pool, conn);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let mut expected = Vec::with_capacity(9);
    for i in 1..9 {
        if let model::Push::Game(model::push::Game::Registered { game, updates }) = handler
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: player.id,
                    opponent: accepted.id,
                    score: 11,
                    opponent_score: i,
                    challenge: false,
                    millis: types::Millis::from(i64::from(i)),
                }),
                true,
            )
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap()
        {
            assert_eq!(updates.len(), 0);
            expected.push(types::Game {
                id: 0,
                created_ms: types::Millis::from(0),
                ..game
            });
        } else {
            panic!()
        }
    }

    sqlx::query!("DELETE FROM games")
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(store.games().list().await.unwrap().len(), 0);

    for i in 1..9 {
        if let model::Push::Game(model::push::Game::Registered { updates, .. }) = handler
            .call(
                model::Request::Game(model::request::Game::Register {
                    player: player.id,
                    opponent: accepted.id,
                    score: 11,
                    opponent_score: 9 - i,
                    challenge: false,
                    millis: types::Millis::from(i64::from(9 - i)),
                }),
                true,
            )
            .await
            .done()
            .unwrap()
            .none()
            .unwrap()
            .some()
            .unwrap()
        {
            assert_eq!(updates.len(), usize::from(i - 1));
        } else {
            panic!()
        }
    }

    let model::Response::Games(response) = handler
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .raw()
        .unwrap()
    else {
        panic!()
    };

    let response = response
        .into_iter()
        .map(|g| types::Game {
            id: 0,
            created_ms: types::Millis::from(0),
            ..types::Game::from(g)
        })
        .filter(|g| !g.deleted)
        .collect::<Vec<_>>();

    assert_eq!(response, expected);
}

#[sqlx::test]
async fn history(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: true,
                millis: now(),
            }),
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

    let original_game = game;

    let model::Push::Game(model::push::Game::Updated { game, .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Update(types::Game {
                score_one: 7,
                score_two: 11,
                ..original_game.clone()
            })),
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

    handler
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .ok(model::Response::Games(vec![game.into()]))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    handler
        .call(
            model::Request::Game(model::request::Game::History(original_game.id)),
            false,
        )
        .await
        .map_ok(
            |r| {
                let model::Response::History(response) = r else {
                    panic!()
                };
                response
                    .into_iter()
                    .map(types::History::from)
                    .map(|h| types::History {
                        id: 0,
                        created_ms: types::Millis::from(0),
                        ..h
                    })
                    .collect()
            },
            vec![types::History {
                id: 0,
                game: original_game.id,
                player_one: original_game.player_one,
                player_two: original_game.player_two,
                score_one: original_game.score_one,
                score_two: original_game.score_two,
                challenge: original_game.challenge,
                deleted: original_game.deleted,
                millis: original_game.millis,
                created_ms: types::Millis::from(0),
            }],
        )
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn history_only_when_relevant(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: true,
                millis: now(),
            }),
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

    let first_game = game;

    let model::Push::Game(model::push::Game::Registered { game, .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: player.id,
                opponent: accepted.id,
                score: 11,
                opponent_score: 0,
                challenge: false,
                millis: now(),
            }),
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

    let second_game = game;
    let second_game_id = second_game.id;

    let model::Push::Game(model::push::Game::Updated { updates, .. }) = handler
        .call(
            model::Request::Game(model::request::Game::Update(types::Game {
                deleted: true,
                ..first_game.clone()
            })),
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
    let modified_second_game = updates.into_iter().next().map(types::Game::from).unwrap();

    assert_ne!(second_game, modified_second_game);

    let first_game = types::Game {
        rating_delta: 0.0,
        deleted: true,
        ..first_game
    };

    handler
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .ok(model::Response::Games(
            [first_game, modified_second_game]
                .map(types::GameTuple::from)
                .into_iter()
                .collect(),
        ))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    handler
        .call(
            model::Request::Game(model::request::Game::History(second_game_id)),
            false,
        )
        .await
        .ok(model::Response::History(Vec::new()))
        .unwrap()
        .none()
        .unwrap()
        .none()
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
        .call(model::Request::Game(model::request::Game::List), false)
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(
            model::Request::Game(model::request::Game::Register {
                player: 0,
                opponent: 0,
                score: 0,
                opponent_score: 0,
                challenge: false,
                millis: super::now(),
            }),
            false,
        )
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(
            model::Request::Game(model::request::Game::History(0)),
            false,
        )
        .await
        .err(model::Error::Forbidden)
        .unwrap();
}
