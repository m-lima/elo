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
}

pub async fn initialize(store: &store::Store) -> Result<(), Error> {
    let auth = access::Auth::new(store.clone());

    populate_users(store, &auth).await?;
    populate_games(store, &auth).await?;

    Ok(())
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
        .migrate(skillratings::elo::EloRating::new().rating)
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
            access::UserAccess::Regular(user) => user,
            access::UserAccess::Pending(user) => {
                let email = user.email().clone();
                let mut handler = handler::Handler::new(user, store.clone(), smtp::Smtp::empty());
                handler
                    .call(model::Request::Invite(model::request::Invite::Accept))
                    .await?;
                get_registered_user(auth, &email).await?
            }
        };

        let mut handler = handler::Handler::new(user, store.clone(), smtp::Smtp::empty());
        for _ in 0..amount {
            let (invitee, amount) = players.next().ok_or(Error::WrongCount)?;
            let (invitee_name, invitee_email) = make_player(invitee);

            match handler
                .call(model::Request::Invite(model::request::Invite::Player {
                    name: invitee_name,
                    email: invitee_email.clone(),
                }))
                .await?
            {
                model::Response::Id(_) => stack.push_back((invitee_email, amount)),
                _ => unreachable!("Unexpected response"),
            };
        }
    }

    Ok(())
}

async fn populate_games(store: &store::Store, auth: &access::Auth) -> Result<(), Error> {
    use rand::Rng;
    use ws::Service;

    let mut rand: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(8855);

    let players = {
        let user = get_registered_user(auth, consts::mock::USER_EMAIL).await?;

        let mut handler = handler::Handler::new(user, store.clone(), smtp::Smtp::empty());
        match handler
            .call(model::Request::Player(model::request::Player::List))
            .await?
        {
            model::Response::Players(players) => players,
            _ => unreachable!("Unexpected response"),
        }
    };

    let distribution =
        rand::distributions::WeightedIndex::new((0..players.len()).map(|i| 1 + i / 4))?;

    for _ in 0..players.len() * 15 {
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

        let model::Response::Id(id) = handler::Handler::new(
            get_registered_user(auth, &user.2).await?,
            store.clone(),
            smtp::Smtp::empty(),
        )
        .call(model::Request::Game(model::request::Game::Register {
            opponent: opponent.0,
            score: user_score,
            opponent_score,
        }))
        .await?
        else {
            unreachable!("Unexpected response")
        };

        if rand.gen_bool(0.99) {
            handler::Handler::new(
                get_registered_user(auth, &opponent.2).await?,
                store.clone(),
                smtp::Smtp::empty(),
            )
            .call(model::Request::Game(model::request::Game::Accept(id)))
            .await?;
        }
    }

    Ok(())
}

async fn get_user(auth: &access::Auth, email: &str) -> Result<access::UserAccess, Error> {
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
            access::UserAccess::Regular(user) => Ok(user),
            access::UserAccess::Pending(_) => Err(Error::PendingUser(String::from(email))),
        })
}
