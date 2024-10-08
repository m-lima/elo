use super::super::{access, broadcaster, handler, model};

use crate::{mailbox, server, smtp, store, types, ws};

type Result<T = ()> = std::result::Result<T, Error>;

pub struct Handler<A>
where
    A: handler::Access,
{
    inner: handler::Handler<A, Smtp>,
    push: tokio::sync::broadcast::Receiver<model::Push>,
    email: tokio::sync::mpsc::Receiver<smtp::Payload>,
}

impl Handler<access::Regular> {
    pub async fn new(user: &str, store: &store::Store) -> Result<Self> {
        let broadcaster = broadcaster::Broadcaster::new();
        let push = broadcaster.subscribe();
        let (smtp, email) = Smtp::new();

        let auth = access::Auth::new(store.clone());
        let user = match server::auth::Provider::auth(&auth, user)
            .await
            .map_err(Error::Store)?
            .ok_or(Error::MissingUser)?
        {
            access::Dynamic::Regular(user) => user,
            access::Dynamic::Pending(_) => return Err(Error::UserAccess),
        };

        let handler = handler::Handler::new(user, store.clone(), broadcaster, smtp);

        Ok(Self {
            inner: handler,
            push,
            email,
        })
    }
}

impl Handler<access::Pending> {
    pub async fn pending(user: &str, store: &store::Store) -> Result<Self> {
        let broadcaster = broadcaster::Broadcaster::new();
        let push = broadcaster.subscribe();
        let (smtp, email) = Smtp::new();

        let auth = access::Auth::new(store.clone());
        let user = match server::auth::Provider::auth(&auth, user)
            .await
            .map_err(Error::Store)?
            .ok_or(Error::MissingUser)?
        {
            access::Dynamic::Pending(user) => user,
            access::Dynamic::Regular(_) => return Err(Error::UserAccess),
        };

        let handler = handler::Handler::new(user, store.clone(), broadcaster, smtp);

        Ok(Self {
            inner: handler,
            push,
            email,
        })
    }
}

impl Handler<access::Regular> {
    pub async fn invite(&mut self, name: &str, email: &str) -> Result<types::Invite> {
        match self
            .call(
                model::Request::Invite(model::request::Invite::Player {
                    name: String::from(name),
                    email: String::from(email),
                }),
                true,
            )
            .await
            .done()?
            .some(smtp::Payload::Invite(
                mailbox::Mailbox::new(String::from(name), String::from(email))
                    .map_err(Error::Mailbox)?,
            ))?
            .some()?
        {
            model::Push::Player(model::push::Player::Invited(invite)) => Ok(invite),
            p => Err(Error::from(p)),
        }
    }

    pub async fn invite_full(
        &mut self,
        player: &types::Player,
        store: &store::Store,
        name: &str,
        email: &str,
    ) -> Result<types::Player> {
        let accepted = self.invite(name, email).await?;

        match Handler::pending(&accepted.email, store)
            .await?
            .call(model::Request::Invite(model::request::Invite::Accept), true)
            .await
            .done()?
            .some(smtp::Payload::InviteOutcome {
                inviter: mailbox::Proto {
                    name: player.name.clone(),
                    email: player.email.clone(),
                },
                invitee: mailbox::Proto {
                    name: String::from(name),
                    email: String::from(email),
                },
                accepted: true,
            })?
            .some()?
        {
            model::Push::Player(model::push::Player::Joined(joined)) => Ok(joined),
            p => Err(Error::from(p)),
        }
    }
}

impl<A> Handler<A>
where
    A: handler::Access,
{
    #[must_use]
    pub async fn call(&mut self, request: model::Request, mutable: bool) -> ResponseVerifier<'_> {
        let Ok(model::Response::Version {
            data: old_version, ..
        }) = ws::Service::call(&mut self.inner, model::Request::Version).await
        else {
            panic!()
        };

        let verifier = ResponseVerifier::new(
            ws::Service::call(&mut self.inner, request).await,
            &mut self.email,
            &mut self.push,
        );

        let Ok(model::Response::Version {
            data: new_version, ..
        }) = ws::Service::call(&mut self.inner, model::Request::Version).await
        else {
            panic!()
        };

        if mutable {
            assert_ne!(new_version, old_version);
        } else {
            assert_eq!(new_version, old_version);
        }

        verifier
    }
}

impl<A> Drop for Handler<A>
where
    A: handler::Access,
{
    fn drop(&mut self) {
        check_empty_push(&mut self.push).unwrap();
        check_empty_email(&mut self.email).unwrap();
    }
}

pub struct ResponseVerifier<'a> {
    response: std::result::Result<model::Response, model::Error>,
    next: EmailVerifier<'a>,
}

impl<'a> ResponseVerifier<'a> {
    fn new(
        response: std::result::Result<model::Response, model::Error>,
        email: &'a mut tokio::sync::mpsc::Receiver<smtp::Payload>,
        push: &'a mut tokio::sync::broadcast::Receiver<model::Push>,
    ) -> Self {
        Self {
            response,
            next: EmailVerifier::new(email, push),
        }
    }

    pub fn done(self) -> Result<EmailVerifier<'a>> {
        self.ok(model::Response::Done)
    }

    pub fn ok(self, expected: model::Response) -> Result<EmailVerifier<'a>> {
        match self.response {
            Ok(r) => Equal::assert(r, expected).map(|()| self.next),
            Err(e) => Err(Error::ResponseError(e)),
        }
    }

    pub fn map_ok<F, T>(self, mapper: F, expected: T) -> Result<EmailVerifier<'a>>
    where
        F: Fn(model::Response) -> T,
        T: std::fmt::Debug,
    {
        match self.response {
            Ok(r) => Equal::assert(mapper(r), expected).map(|()| self.next),
            Err(e) => Err(Error::ResponseError(e)),
        }
    }

    pub fn err(self, expected: model::Error) -> Result {
        match self.response {
            Ok(r) => Err(Error::UnexpectedResponse(r)),
            Err(e) => Equal::assert(e, expected)
                .map(|()| self.next)
                .and_then(EmailVerifier::none)
                .and_then(PushVerifier::none),
        }
    }

    pub fn raw(self) -> std::result::Result<model::Response, model::Error> {
        self.response
    }
}

pub struct EmailVerifier<'a> {
    email: &'a mut tokio::sync::mpsc::Receiver<smtp::Payload>,
    next: PushVerifier<'a>,
}

impl<'a> EmailVerifier<'a> {
    fn new(
        email: &'a mut tokio::sync::mpsc::Receiver<smtp::Payload>,
        push: &'a mut tokio::sync::broadcast::Receiver<model::Push>,
    ) -> Self {
        Self {
            email,
            next: PushVerifier::new(push),
        }
    }

    pub fn some(self, expected: smtp::Payload) -> Result<PushVerifier<'a>> {
        match self.email.try_recv() {
            Ok(p) => Equal::assert(p, expected).map(|()| self.next),
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => Err(Error::NoMessage),
            e @ Err(_) => Err(Error::BadChannel(format!("{e:?}"))),
        }
    }

    pub fn none(self) -> Result<PushVerifier<'a>> {
        check_empty_email(self.email).map(|()| self.next)
    }
}

pub struct PushVerifier<'a> {
    push: &'a mut tokio::sync::broadcast::Receiver<model::Push>,
}

impl<'a> PushVerifier<'a> {
    fn new(push: &'a mut tokio::sync::broadcast::Receiver<model::Push>) -> Self {
        Self { push }
    }

    pub fn some(self) -> Result<model::Push> {
        match self.push.try_recv() {
            Ok(p) => Ok(p),
            Err(tokio::sync::broadcast::error::TryRecvError::Empty) => Err(Error::NoMessage),
            e @ Err(_) => Err(Error::BadChannel(format!("{e:?}"))),
        }
    }

    pub fn none(self) -> Result {
        check_empty_push(self.push)
    }
}

fn check_empty_push(push: &mut tokio::sync::broadcast::Receiver<model::Push>) -> Result {
    match push.try_recv() {
        Err(tokio::sync::broadcast::error::TryRecvError::Empty) => Ok(()),
        Ok(p) => Err(Error::from(p)),
        e @ Err(_) => Err(Error::BadChannel(format!("{e:?}"))),
    }
}

fn check_empty_email(email: &mut tokio::sync::mpsc::Receiver<smtp::Payload>) -> Result {
    match email.try_recv() {
        Err(tokio::sync::mpsc::error::TryRecvError::Empty) => Ok(()),
        Ok(p) => Err(Error::UnexpectedEmail(p)),
        e @ Err(_) => Err(Error::BadChannel(format!("{e:?}"))),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Store(store::Error),
    #[error(transparent)]
    Mailbox(mailbox::Error),
    #[error("Missing user")]
    MissingUser,
    #[error("Wrong user access")]
    UserAccess,
    #[error("Error response: {0:?}")]
    #[allow(clippy::enum_variant_names)]
    ResponseError(model::Error),
    #[error("Unexpected response: {0:?}")]
    UnexpectedResponse(model::Response),
    #[error("Unexpected push: {0}")]
    UnexpectedPush(String),
    #[error("Unexpected email: {0:?}")]
    UnexpectedEmail(smtp::Payload),
    #[error("No messages in queue")]
    NoMessage,
    #[error("Message queue in bad state: {0}")]
    BadChannel(String),
    #[error("{0}")]
    NotEqual(String),
}

impl From<model::Push> for Error {
    fn from(value: model::Push) -> Self {
        Self::UnexpectedPush(format!("{value:?}"))
    }
}

struct Equal;

impl Equal {
    fn assert<V, E>(value: V, expected: E) -> Result
    where
        V: std::fmt::Debug,
        E: std::fmt::Debug,
    {
        let value = format!("{value:?}");
        let expected = format!("{expected:?}");
        if value == expected {
            Ok(())
        } else {
            eprintln!(
                r#"Values differ
{value}
{expected}"#
            );
            Err(Error::NotEqual(format!(
                r#"Values differ
{value}
{expected}"#
            )))
        }
    }
}

#[derive(Clone)]
struct Smtp {
    tx: tokio::sync::mpsc::Sender<smtp::Payload>,
}

impl Smtp {
    fn new() -> (Self, tokio::sync::mpsc::Receiver<smtp::Payload>) {
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        (Self { tx }, rx)
    }
}

impl smtp::Smtp for Smtp {
    async fn send(&mut self, payload: smtp::Payload) {
        self.tx.send(payload).await.unwrap();
    }
}
