use super::*;
use crate::mailbox;

#[sqlx::test]
async fn ok(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;
}

#[sqlx::test]
async fn normalization(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    handler
        .call_done(model::Request::Invite(model::request::Invite::Player {
            name: format!("{WHITE_SPACE}{INVITED_NAME}{WHITE_SPACE}"),
            email: format!("{WHITE_SPACE}iNviTeD@eMAil.cOm{WHITE_SPACE}"),
        }))
        .await;

    handler.check_invite(INVITED_NAME, INVITED_EMAIL, player.id);
}

#[sqlx::test]
async fn accept(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    let invited = handler
        .invite(INVITED_NAME, INVITED_EMAIL, player.id)
        .await
        .unwrap();

    RichHandler::pending(&invited.email, &store)
        .await
        .accept(&player, &invited)
        .await;
}

#[sqlx::test]
async fn reject(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    let invited = handler
        .invite(INVITED_NAME, INVITED_EMAIL, player.id)
        .await
        .unwrap();

    let mut handler = RichHandler::pending(&invited.email, &store).await;

    handler
        .call_done(model::Request::Invite(model::request::Invite::Reject))
        .await;

    match handler.email.try_recv().unwrap() {
        smtp::Payload::InviteOutcome {
            inviter,
            invitee,
            accepted,
        } => {
            assert_eq!(inviter.name, TESTER_NAME);
            assert_eq!(inviter.email, TESTER_EMAIL);
            assert_eq!(invitee.name, INVITED_NAME);
            assert_eq!(invitee.email, INVITED_EMAIL);
            assert!(!accepted);
        }
        p @ smtp::Payload::Invite(_) => panic!("Unexpected email: {p:?}"),
    }

    match handler.push.try_recv().unwrap() {
        model::Push::Player(model::push::Player::Uninvited(uninvited)) => {
            assert_eq!(invited, uninvited);
        }
        p => panic!("Unexpected email: {p:?}"),
    }
}

#[sqlx::test]
async fn cancel(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;

    handler
        .call_done(model::Request::Invite(model::request::Invite::Cancel(
            invited.id,
        )))
        .await;

    handler.check_no_email();

    match handler.push.try_recv().unwrap() {
        model::Push::Player(model::push::Player::Uninvited(uninvited)) => {
            assert_eq!(invited, uninvited);
        }
        p => panic!("Unexpected email: {p:?}"),
    }
}

#[sqlx::test]
async fn invalid_input(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: String::new(),
            email: String::from(INVITED_EMAIL),
        }))
        .await
    {
        model::Error::InvalidEmail(mailbox::Error::MissingName) => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: String::from(WHITE_SPACE),
            email: String::from(INVITED_EMAIL),
        }))
        .await
    {
        model::Error::InvalidEmail(mailbox::Error::MissingName) => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: String::from(INVITED_NAME),
            email: String::new(),
        }))
        .await
    {
        model::Error::InvalidEmail(mailbox::Error::Address(
            lettre::address::AddressError::MissingParts,
        )) => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: String::from(INVITED_NAME),
            email: String::from(WHITE_SPACE),
        }))
        .await
    {
        model::Error::InvalidEmail(mailbox::Error::Address(
            lettre::address::AddressError::MissingParts,
        )) => {}
        e => panic!("Unexpected error: {e:?}"),
    }
}

#[sqlx::test]
async fn repeated_input(pool: sqlx::sqlite::SqlitePool) {
    let (player, store) = init(&pool).await;
    let mut handler = RichHandler::new(&player.email, &store).await;

    handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: String::from(TESTER_NAME),
            email: String::from(TESTER_EMAIL),
        }))
        .await
    {
        model::Error::Store(store::Error::AlreadyExists) => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: format!("{WHITE_SPACE}{TESTER_NAME}{WHITE_SPACE}"),
            email: format!("{WHITE_SPACE}tEsTEr@eMAil.cOm{WHITE_SPACE}"),
        }))
        .await
    {
        model::Error::Store(store::Error::AlreadyExists) => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: String::from(INVITED_NAME),
            email: String::from(INVITED_EMAIL),
        }))
        .await
    {
        model::Error::Store(store::Error::AlreadyExists) => {}
        e => panic!("Unexpected error: {e:?}"),
    }

    match handler
        .call_err(model::Request::Invite(model::request::Invite::Player {
            name: format!("{WHITE_SPACE}invited{WHITE_SPACE}"),
            email: format!("{WHITE_SPACE}iNviTeD@eMAil.cOm{WHITE_SPACE}"),
        }))
        .await
    {
        model::Error::Store(store::Error::AlreadyExists) => {}
        e => panic!("Unexpected error: {e:?}"),
    }
}
