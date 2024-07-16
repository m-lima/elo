use super::{super::model, *};
use crate::{mailbox, smtp};

#[sqlx::test]
async fn ok(pool: sqlx::sqlite::SqlitePool) {
    let (player, _, mut handler) = init!(pool);
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    assert_eq!(invited.name, INVITED_NAME);
    assert_eq!(invited.email, INVITED_EMAIL);
    assert_eq!(invited.inviter, player.id);
}

#[sqlx::test]
async fn normalization(pool: sqlx::sqlite::SqlitePool) {
    let (player, _, mut handler) = init!(pool);
    let model::Push::Player(model::push::Player::Invited(invited)) = handler
        .call(model::Request::Invite(model::request::Invite::Player {
            name: format!("{WHITE_SPACE}{INVITED_NAME}{WHITE_SPACE}"),
            email: format!("{WHITE_SPACE}iNviTeD@eMAil.cOm{WHITE_SPACE}"),
        }))
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
async fn accept(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    let mut handler = framework::Handler::pending(&invited.email, &store)
        .await
        .unwrap();

    let joined = handler.accept(&player, &invited).await.unwrap();

    assert_eq!(joined.name, invited.name);
    assert_eq!(joined.email, invited.email);
    assert_eq!(joined.inviter, Some(player.id));
    assert!((joined.rating - skillratings::elo::EloRating::new().rating).abs() < f64::EPSILON);
}

#[sqlx::test]
async fn reject(pool: sqlx::sqlite::SqlitePool) {
    let (player, store, mut handler) = init!(pool);
    let invited = handler.invite(INVITED_NAME, INVITED_EMAIL).await.unwrap();

    let mut handler = framework::Handler::pending(&invited.email, &store)
        .await
        .unwrap();

    let model::Push::Player(model::push::Player::Uninvited(uninvited)) = handler
        .call(model::Request::Invite(model::request::Invite::Reject))
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

// #[sqlx::test]
// async fn cancel(pool: sqlx::sqlite::SqlitePool) {
//     let (player, store) = init(&pool).await;
//     let mut handler = RichHandler::new(&player.email, &store).await;
//
//     let invited = handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;
//
//     handler
//         .call_done(model::Request::Invite(model::request::Invite::Cancel(
//             invited.id,
//         )))
//         .await;
//
//     handler.check_no_email();
//
//     match handler.push.try_recv().unwrap() {
//         model::Push::Player(model::push::Player::Uninvited(uninvited)) => {
//             assert_eq!(invited, uninvited);
//         }
//         p => panic!("Unexpected email: {p:?}"),
//     }
// }
//
// #[sqlx::test]
// async fn invalid_input(pool: sqlx::sqlite::SqlitePool) {
//     let (player, store) = init(&pool).await;
//     let mut handler = RichHandler::new(&player.email, &store).await;
//
//     match handler
//         .call_err(model::Request::Invite(model::request::Invite::Player {
//             name: String::new(),
//             email: String::from(INVITED_EMAIL),
//         }))
//         .await
//     {
//         model::Error::InvalidEmail(mailbox::Error::MissingName) => {}
//         e => panic!("Unexpected error: {e:?}"),
//     }
//
//     match handler
//         .call_err(model::Request::Invite(model::request::Invite::Player {
//             name: String::from(WHITE_SPACE),
//             email: String::from(INVITED_EMAIL),
//         }))
//         .await
//     {
//         model::Error::InvalidEmail(mailbox::Error::MissingName) => {}
//         e => panic!("Unexpected error: {e:?}"),
//     }
//
//     match handler
//         .call_err(model::Request::Invite(model::request::Invite::Player {
//             name: String::from(INVITED_NAME),
//             email: String::new(),
//         }))
//         .await
//     {
//         model::Error::InvalidEmail(mailbox::Error::Address(
//             lettre::address::AddressError::MissingParts,
//         )) => {}
//         e => panic!("Unexpected error: {e:?}"),
//     }
//
//     match handler
//         .call_err(model::Request::Invite(model::request::Invite::Player {
//             name: String::from(INVITED_NAME),
//             email: String::from(WHITE_SPACE),
//         }))
//         .await
//     {
//         model::Error::InvalidEmail(mailbox::Error::Address(
//             lettre::address::AddressError::MissingParts,
//         )) => {}
//         e => panic!("Unexpected error: {e:?}"),
//     }
// }
//
// #[sqlx::test]
// async fn repeated_input(pool: sqlx::sqlite::SqlitePool) {
//     let (player, store) = init(&pool).await;
//     let mut handler = RichHandler::new(&player.email, &store).await;
//
//     handler.invite(INVITED_NAME, INVITED_EMAIL, player.id).await;
//
//     match handler
//         .call_err(model::Request::Invite(model::request::Invite::Player {
//             name: String::from(TESTER_NAME),
//             email: String::from(TESTER_EMAIL),
//         }))
//         .await
//     {
//         model::Error::Store(store::Error::AlreadyExists) => {}
//         e => panic!("Unexpected error: {e:?}"),
//     }
//
//     match handler
//         .call_err(model::Request::Invite(model::request::Invite::Player {
//             name: format!("{WHITE_SPACE}{TESTER_NAME}{WHITE_SPACE}"),
//             email: format!("{WHITE_SPACE}tEsTEr@eMAil.cOm{WHITE_SPACE}"),
//         }))
//         .await
//     {
//         model::Error::Store(store::Error::AlreadyExists) => {}
//         e => panic!("Unexpected error: {e:?}"),
//     }
//
//     match handler
//         .call_err(model::Request::Invite(model::request::Invite::Player {
//             name: String::from(INVITED_NAME),
//             email: String::from(INVITED_EMAIL),
//         }))
//         .await
//     {
//         model::Error::Store(store::Error::AlreadyExists) => {}
//         e => panic!("Unexpected error: {e:?}"),
//     }
//
//     match handler
//         .call_err(model::Request::Invite(model::request::Invite::Player {
//             name: format!("{WHITE_SPACE}invited{WHITE_SPACE}"),
//             email: format!("{WHITE_SPACE}iNviTeD@eMAil.cOm{WHITE_SPACE}"),
//         }))
//         .await
//     {
//         model::Error::Store(store::Error::AlreadyExists) => {}
//         e => panic!("Unexpected error: {e:?}"),
//     }
// }
