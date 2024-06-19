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
}

pub async fn initialize(store: store::Store) -> Result<(), Error> {
    use ws::Service;

    let user = {
        let rating = skillratings::glicko2::Glicko2Rating::new();
        store
            .migrate(rating.rating, rating.deviation, rating.volatility)
            .await
    }?;

    let auth = access::Auth::new(store.clone());

    // invite::Invite::new(self).handle(model::Invite::Player())

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

        let user = match get_user(&auth, &user).await? {
            access::UserAccess::Regular(user) => user,
            access::UserAccess::Pending(user) => {
                let email = user.email().clone();
                let mut handler = handler::Handler::new(user, store.clone(), smtp::Smtp::empty());
                handler
                    .call(model::Request::Invite(model::Invite::Accept))
                    .await?;

                match get_user(&auth, &email).await? {
                    access::UserAccess::Regular(user) => user,
                    access::UserAccess::Pending(_) => return Err(Error::PendingUser(email)),
                }
            }
        };

        let mut handler = handler::Handler::new(user, store.clone(), smtp::Smtp::empty());
        for _ in 0..amount {
            let (invitee, amount) = players.next().ok_or(Error::WrongCount)?;
            let invitee = make_invite(invitee);
            let email = invitee.email.clone();

            match handler
                .call(model::Request::Invite(model::Invite::Player(invitee)))
                .await?
            {
                model::Response::Id(_) => stack.push_back((email, amount)),
                _ => unreachable!("Unexpected response"),
            };
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

fn make_invite(name: &str) -> model::InvitePlayer {
    let email = name.to_lowercase().replace(' ', ".");
    let email = format!("{email}@email.com");
    let name = String::from(name);

    model::InvitePlayer { name, email }
}
