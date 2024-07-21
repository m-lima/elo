use super::{access, handler, model};
use crate::{consts, server, smtp, store, types, ws};

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
}

pub async fn initialize(store: &store::Store, count: u16) -> Result<(), Error> {
    let auth = access::Auth::new(store.clone());

    populate_users(store, &auth).await?;
    populate_games(store, &auth, count).await
}

async fn populate_users(store: &store::Store, auth: &access::Auth) -> Result<(), Error> {
    use ws::Service;

    fn make_player(name: &str) -> (String, String) {
        let email = name.to_lowercase().replace(' ', ".");
        let email = format!("{email}@email.com");
        let name = String::from(name);

        (name, email)
    }

    let user = store.initialize().await?;

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

async fn populate_games(
    store: &store::Store,
    auth: &access::Auth,
    count: u16,
) -> Result<(), Error> {
    use rand::Rng;
    use ws::Service;

    let mut rand: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(8855);

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

    let millis = {
        let initial = 1_706_702_400_000_i64; // 2024-01-31 12:00:00
        let mut millis = (0..players.len() * usize::from(count))
            .scan(initial, |acc, _| {
                *acc -= rand.gen_range((2 * 60 * 1000)..(2 * 60 * 60 * 1000));
                Some(*acc)
            })
            .collect::<Vec<_>>();
        millis.reverse();
        millis
    };

    for millis in millis {
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

        match handler
            .call(model::Request::Game(model::request::Game::Register {
                player: user.0,
                opponent: opponent.0,
                score: user_score,
                opponent_score,
                challenge,
                millis: types::Millis::from(millis),
            }))
            .await
        {
            Ok(model::Response::Done) => {}
            Ok(_) => unreachable!("Unexpected response"),
            Err(model::Error::Store(store::Error::InvalidValue(
                "Players cannot challenge each other more than once a day",
            ))) if challenge => {
                let model::Response::Done = handler
                    .call(model::Request::Game(model::request::Game::Register {
                        player: user.0,
                        opponent: opponent.0,
                        score: user_score,
                        opponent_score,
                        challenge: false,
                        millis: types::Millis::from(millis),
                    }))
                    .await?
                else {
                    unreachable!("Unexpected response");
                };
            }
            Err(err) => return Err(err.into()),
        };
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
