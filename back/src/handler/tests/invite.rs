use super::{super::model, *};
use crate::{mailbox, smtp};

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;

#[sqlx::test]
async fn list(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);

    handler
        .call(model::Request::Invite(model::request::Invite::List), false)
        .await
        .ok(model::Response::Invites(Vec::new()))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    handler
        .call(model::Request::Invite(model::request::Invite::List), false)
        .await
        .ok(model::Response::Invites(
            [invited.clone()]
                .map(types::InviteTuple::from)
                .into_iter()
                .collect(),
        ))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();

    framework::Handler::pending(&invited.email, &store)
        .await
        .unwrap()
        .call(model::Request::Invite(model::request::Invite::Accept), true)
        .await
        .done()
        .unwrap()
        .some(smtp::Payload::InviteOutcome {
            inviter: mailbox::Proto {
                name: player.name.clone(),
                email: player.email.clone(),
            },
            invitee: mailbox::Proto {
                name: invited.name.clone(),
                email: invited.email.clone(),
            },
            accepted: true,
        })
        .unwrap()
        .some()
        .unwrap();

    handler
        .call(model::Request::Invite(model::request::Invite::List), false)
        .await
        .ok(model::Response::Invites(Vec::new()))
        .unwrap()
        .none()
        .unwrap()
        .none()
        .unwrap();
}

#[sqlx::test]
async fn ok(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, _, mut handler, _) = init!(pool, conn);
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    assert_eq!(invited.name, INVITED_NAME);
    assert_eq!(invited.email, INVITED_EMAIL);
    assert_eq!(invited.inviter, player.id);
}

#[sqlx::test]
async fn normalization(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, _, mut handler, _) = init!(pool, conn);
    let model::Push::Player(model::push::Player::Invited(invited)) = handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: format!("{WHITE_SPACE}{INVITED_NAME}{WHITE_SPACE}"),
                email: format!("{WHITE_SPACE}iNviTeD@eMAil.cOm{WHITE_SPACE}"),
            }),
            true,
        )
        .await
        .done()
        .unwrap()
        .some(smtp::Payload::Invite(
            mailbox::Mailbox::new(String::from(INVITED_NAME), String::from(INVITED_EMAIL)).unwrap(),
        ))
        .unwrap()
        .some()
        .unwrap()
    else {
        panic!()
    };

    assert_eq!(invited.name, INVITED_NAME);
    assert_eq!(invited.email, INVITED_EMAIL);
    assert_eq!(invited.inviter, player.id);
}

#[sqlx::test]
async fn accept(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);
    let accepted = handler
        .invite_full(&player, &store, ACCEPTED_NAME, ACCEPTED_EMAIL)
        .await
        .unwrap();

    assert_eq!(accepted.name, ACCEPTED_NAME);
    assert_eq!(accepted.email, ACCEPTED_EMAIL);
    assert_eq!(accepted.inviter, Some(player.id));
}

#[sqlx::test]
async fn reject(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let (player, store, mut handler, _) = init!(pool, conn);
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    let mut handler = framework::Handler::pending(&invited.email, &store)
        .await
        .unwrap();

    let model::Push::Player(model::push::Player::Uninvited(uninvited)) = handler
        .call(model::Request::Invite(model::request::Invite::Reject), true)
        .await
        .done()
        .unwrap()
        .some(smtp::Payload::InviteOutcome {
            inviter: mailbox::Proto {
                name: player.name.clone(),
                email: player.email.clone(),
            },
            invitee: mailbox::Proto {
                name: invited.name.clone(),
                email: invited.email.clone(),
            },
            accepted: false,
        })
        .unwrap()
        .some()
        .unwrap()
    else {
        panic!()
    };

    assert_eq!(invited, uninvited);
}

#[sqlx::test]
async fn cancel(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let mut handler = init!(pool, conn).2;
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    let model::Push::Player(model::push::Player::Uninvited(uninvited)) = handler
        .call(
            model::Request::Invite(model::request::Invite::Cancel(invited.id)),
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

    assert_eq!(invited, uninvited);
}

#[sqlx::test]
async fn cancel_not_found(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let mut handler = init!(pool, conn).2;

    handler
        .call(
            model::Request::Invite(model::request::Invite::Cancel(27)),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::NotFound))
        .unwrap();
}

#[sqlx::test]
async fn invalid_input(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let mut handler = init!(pool, conn).2;

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: String::new(),
                email: String::from(INVITED_EMAIL),
            }),
            false,
        )
        .await
        .err(model::Error::InvalidEmail(mailbox::Error::MissingName))
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: String::from(WHITE_SPACE),
                email: String::from(INVITED_EMAIL),
            }),
            false,
        )
        .await
        .err(model::Error::InvalidEmail(mailbox::Error::MissingName))
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: String::from(INVITED_NAME),
                email: String::new(),
            }),
            false,
        )
        .await
        .err(model::Error::InvalidEmail(mailbox::Error::Address(
            lettre::address::AddressError::MissingParts,
        )))
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: String::from(INVITED_NAME),
                email: String::from(WHITE_SPACE),
            }),
            false,
        )
        .await
        .err(model::Error::InvalidEmail(mailbox::Error::Address(
            lettre::address::AddressError::MissingParts,
        )))
        .unwrap();
}

#[sqlx::test]
async fn repeated_input(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let mut handler = init!(pool, conn).2;

    handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: String::from(TESTER_NAME),
                email: String::from(TESTER_EMAIL),
            }),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: format!("{WHITE_SPACE}{TESTER_NAME}{WHITE_SPACE}"),
                email: format!("{WHITE_SPACE}tEsTEr@eMAil.cOm{WHITE_SPACE}"),
            }),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: String::from(INVITED_NAME),
                email: String::from(INVITED_EMAIL),
            }),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: format!("{WHITE_SPACE}invited{WHITE_SPACE}"),
                email: format!("{WHITE_SPACE}iNviTeD@eMAil.cOm{WHITE_SPACE}"),
            }),
            false,
        )
        .await
        .err(model::Error::Store(store::Error::AlreadyExists))
        .unwrap();
}

#[sqlx::test]
async fn forbidden_regular(pool: SqlitePoolOptions, conn: SqliteConnectOptions) {
    let mut handler = init!(pool, conn).2;

    handler
        .call(
            model::Request::Invite(model::request::Invite::Accept),
            false,
        )
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Reject),
            false,
        )
        .await
        .err(model::Error::Forbidden)
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
        .call(model::Request::Invite(model::request::Invite::List), false)
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Cancel(0)),
            false,
        )
        .await
        .err(model::Error::Forbidden)
        .unwrap();

    handler
        .call(
            model::Request::Invite(model::request::Invite::Player {
                name: String::new(),
                email: String::new(),
            }),
            false,
        )
        .await
        .err(model::Error::Forbidden)
        .unwrap();
}
