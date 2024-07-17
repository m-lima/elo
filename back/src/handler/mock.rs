use super::{access, handler, model};
use crate::{consts, server, smtp, store, ws};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to migrate: {0:?}")]
    Migration(#[from] sqlx::Error),
    #[error(transparent)]
    Store(#[from] store::Error),
    #[error("Handler error: {0:?}")]
    Handler(#[from] model::Error),
    #[error("Could not find user `{0}`")]
    NotFound(String),
    #[error("User is still `Pending` after accepting invitation")]
    PendingUser(String),
    #[error("Expected more users, but the list of names is empty")]
    WrongCount,
    #[error("Could not build distribution: {0:?}")]
    Distribution(#[from] rand::distributions::WeightedError),
    #[error("Could not receive push: {0:?}")]
    Push(#[from] tokio::sync::broadcast::error::RecvError),
    #[error("Could not fetch data to adjust dates: {0:?}")]
    FetchAdjustDates(sqlx::Error),
    #[error("Could not update data to adjust dates: {0:?}")]
    UpdateAdjustDates(sqlx::Error),
}

#[derive(sqlx::FromRow)]
struct Created {
    created_ms: i64,
}

pub async fn initialize(store: &store::Store, count: u16) -> Result<(), Error> {
    let auth = access::Auth::new(store.clone());

    let mut rand: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(8855);

    populate(store, &auth, &mut rand, count).await?;
    adjust_dates(store, &mut rand).await?;

    Ok(())
}

async fn populate<R>(
    store: &store::Store,
    auth: &access::Auth,
    rand: &mut R,
    count: u16,
) -> Result<(), Error>
where
    R: rand::Rng,
{
    populate_users(store, auth).await?;
    populate_games(store, auth, rand, count).await
}

async fn populate_users(store: &store::Store, auth: &access::Auth) -> Result<(), Error> {
    use ws::Service;

    fn make_player(name: &str) -> (String, String) {
        let email = name.to_lowercase().replace(' ', ".");
        let email = format!("{email}@email.com");
        let name = String::from(name);

        (name, email)
    }

    let user = store
        .initialize(skillratings::elo::EloRating::new().rating)
        .await?;

    // USER
    // |- A
    // |  |- E
    // |  |- F
    // |  |- G
    // |
    // |- B
    // |  |- H
    // |  |  |- O
    // |  |  |- P *pending
    // |  |
    // |  |- I
    // |  |  |- Q
    // |  |  |- R
    // |  |
    // |  |- J
    // |
    // |- C
    // |  |- K
    // |  |  |- S *pending
    // |  |
    // |  |- L
    // |     |- T
    // |
    // |- D
    //    |- M
    //    |- N

    let mut players = consts::mock::NAMES
        .into_iter()
        .zip([3, 3, 2, 2, 0, 0, 0, 2, 2, 0, 1, 1, 0, 0, 0, -1, 0, 0, -1, 0]);

    let mut stack = std::collections::VecDeque::with_capacity(8);
    stack.push_back((user.email, 4));

    while let Some((user, amount)) = stack.pop_front() {
        if amount < 0 {
            continue;
        }

        let user = match get_user(auth, &user).await? {
            access::Dynamic::Regular(user) => user,
            access::Dynamic::Pending(user) => {
                let email = user.email().clone();
                let mut handler = handler::Handler::new(
                    user,
                    store.clone(),
                    super::Broadcaster::new(),
                    smtp::Sender::empty(),
                );
                handler
                    .call(model::Request::Invite(model::request::Invite::Accept))
                    .await?;
                get_registered_user(auth, &email).await?
            }
        };

        let mut handler = handler::Handler::new(
            user,
            store.clone(),
            super::Broadcaster::new(),
            smtp::Sender::empty(),
        );
        for _ in 0..amount {
            let (invitee, amount) = players.next().ok_or(Error::WrongCount)?;
            let (invitee_name, invitee_email) = make_player(invitee);

            let model::Response::Done = handler
                .call(model::Request::Invite(model::request::Invite::Player {
                    name: invitee_name,
                    email: invitee_email.clone(),
                }))
                .await?
            else {
                unreachable!("Unexpected response")
            };

            stack.push_back((invitee_email, amount));
        }
    }

    Ok(())
}

async fn populate_games<R>(
    store: &store::Store,
    auth: &access::Auth,
    rand: &mut R,
    count: u16,
) -> Result<(), Error>
where
    R: rand::Rng,
{
    use ws::Service;

    let players = {
        let user = get_registered_user(auth, consts::mock::USER_EMAIL).await?;

        let mut handler = handler::Handler::new(
            user,
            store.clone(),
            super::Broadcaster::new(),
            smtp::Sender::empty(),
        );
        let model::Response::Players(players) = handler
            .call(model::Request::Player(model::request::Player::List))
            .await?
        else {
            unreachable!("Unexpected response")
        };
        players
    };

    let distribution =
        rand::distributions::WeightedIndex::new((0..players.len()).map(|i| 1 + i / 4))?;

    for _ in 0..players.len() * usize::from(count) {
        let (user, opponent) = {
            let one = rand.sample(&distribution);
            let two = {
                let mut player = rand.sample(&distribution);
                while player == one {
                    player = rand.sample(&distribution);
                }
                player
            };

            (&players[one], &players[two])
        };

        let winner_score = if rand.gen_bool(0.1) { 12 } else { 11 };
        let loser_score = if winner_score == 12 {
            10
        } else {
            rand.gen_range(0..10)
        };

        let (user_score, opponent_score) = if rand.gen_bool(0.5) {
            (winner_score, loser_score)
        } else {
            (loser_score, winner_score)
        };

        let challenge = rand.gen_bool(0.1);

        let mut handler = handler::Handler::new(
            get_registered_user(auth, &user.2).await?,
            store.clone(),
            super::Broadcaster::new(),
            smtp::Sender::empty(),
        );

        let model::Response::Done = handler
            .call(model::Request::Game(model::request::Game::Register {
                opponent: opponent.0,
                score: user_score,
                opponent_score,
                challenge,
            }))
            .await?
        else {
            unreachable!("Unexpected response")
        };
    }

    Ok(())
}

async fn adjust_dates<R>(store: &store::Store, rand: &mut R) -> Result<(), Error>
where
    R: rand::Rng,
{
    let pool = store.raw_pool();

    let initial_date = 1_696_118_400_000;
    let initial_date = adjust_player_dates(pool, initial_date, rand).await?;
    adjust_game_dates(pool, initial_date, rand).await
}

async fn adjust_player_dates<R>(
    pool: &sqlx::SqlitePool,
    mut initial_date: i64,
    rand: &mut R,
) -> Result<i64, Error>
where
    R: rand::Rng,
{
    use crate::types;

    #[derive(sqlx::FromRow)]
    struct Player {
        id: types::Id,
        pending: bool,
    }

    let players = sqlx::query_as!(
        Player,
        r#"
        SELECT
            id,
            pending AS "pending: _"
        FROM
            (
                SELECT
                    id,
                    created_ms,
                    FALSE AS pending
                FROM
                    players
                UNION
                SELECT
                    id,
                    created_ms,
                    TRUE AS pending
                FROM
                    invites
            )
        ORDER BY
            created_ms ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(Error::FetchAdjustDates)?;

    for player in players {
        let created_ms = initial_date + rand.gen_range((2 * 60 * 1000)..(5 * 24 * 60 * 60 * 1000));

        let created = if player.pending {
            sqlx::query_as!(
                Created,
                r#"
                UPDATE
                    invites
                SET
                    created_ms = $2
                WHERE
                    id = $1
                RETURNING
                    created_ms AS "created_ms!: _"
                "#,
                player.id,
                created_ms
            )
            .fetch_one(pool)
            .await
            .map_err(Error::UpdateAdjustDates)?
        } else {
            sqlx::query_as!(
                Created,
                r#"
                UPDATE
                    players
                SET
                    created_ms = $2
                WHERE
                    id = $1
                RETURNING
                    created_ms AS "created_ms!: _"
                "#,
                player.id,
                created_ms
            )
            .fetch_one(pool)
            .await
            .map_err(Error::UpdateAdjustDates)?
        };

        initial_date = created.created_ms;
    }

    Ok(initial_date)
}

async fn adjust_game_dates<R>(
    pool: &sqlx::SqlitePool,
    mut initial_date: i64,
    rand: &mut R,
) -> Result<(), Error>
where
    R: rand::Rng,
{
    use crate::types;

    #[derive(sqlx::FromRow)]
    struct Game {
        id: types::Id,
    }

    let games = sqlx::query_as!(
        Game,
        r#"
        SELECT
            id
        FROM
            games
        ORDER BY
            created_ms ASC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(Error::FetchAdjustDates)?;

    for game in games {
        let created_ms = initial_date + rand.gen_range((2 * 60 * 1000)..(60 * 60 * 1000));

        let created = sqlx::query_as!(
            Created,
            r#"
            UPDATE
                games
            SET
                created_ms = $2
            WHERE
                id = $1
            RETURNING
                created_ms AS "created_ms!: _"
            "#,
            game.id,
            created_ms
        )
        .fetch_one(pool)
        .await
        .map_err(Error::UpdateAdjustDates)?;

        initial_date = created.created_ms;
    }

    Ok(())
}

async fn get_user(auth: &access::Auth, email: &str) -> Result<access::Dynamic, Error> {
    use server::auth::Provider;

    auth.auth(email)
        .await?
        .ok_or(Error::NotFound(String::from(email)))
}

async fn get_registered_user(
    auth: &access::Auth,
    email: &str,
) -> Result<access::User<access::Regular>, Error> {
    use server::auth::Provider;

    auth.auth(email)
        .await?
        .ok_or(Error::NotFound(String::from(email)))
        .and_then(|u| match u {
            access::Dynamic::Regular(user) => Ok(user),
            access::Dynamic::Pending(_) => Err(Error::PendingUser(String::from(email))),
        })
}
