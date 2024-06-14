mod invite;
mod player;

use super::{broadcaster, model};
use crate::{mailbox, smtp, store, types, ws};

pub trait Access: Sized {
    fn into_proto(self) -> mailbox::Proto;

    fn email(&self) -> &String;

    fn handle(
        handler: &mut Handler<Self>,
        request: model::Request,
    ) -> impl std::future::Future<Output = Result<model::Response, model::Error>>;
}

macro_rules! impl_user {
    ($type: ty) => {
        impl Access for $type {
            fn into_proto(self) -> mailbox::Proto {
                mailbox::Proto {
                    name: self.name,
                    email: self.email,
                }
            }

            fn email(&self) -> &String {
                &self.email
            }

            async fn handle(
                handler: &mut Handler<Self>,
                request: model::Request,
            ) -> Result<model::Response, model::Error> {
                match request {
                    model::Request::Player(request) => {
                        player::Player::<Self>::new(handler).handle(request).await
                    }
                    model::Request::Invite(request) => {
                        invite::Invite::<Self>::new(handler).handle(request).await
                    }
                }
            }
        }

        impl super::Access for $type {}
    };
}

impl_user!(types::ExistingUser);
impl_user!(types::PendingUser);

#[derive(Debug)]
pub struct Handler<A>
where
    A: Access,
{
    user: A,
    store: store::Store,
    smtp: smtp::Smtp,
    broadcaster: broadcaster::Broadcaster<model::Push>,
}

impl<A> Handler<A>
where
    A: Access,
{
    #[must_use]
    pub fn new(user: A, store: store::Store, smtp: smtp::Smtp) -> Self {
        let broadcaster = broadcaster::Broadcaster::new();

        Self {
            user,
            store,
            smtp,
            broadcaster,
        }
    }
}

impl<A> ws::Service for Handler<A>
where
    A: Access,
{
    type Request = model::Request;
    type Response = model::Response;
    type Error = model::Error;
    type Push = model::Push;

    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<Self::Push> {
        self.broadcaster.subscribe()
    }

    async fn call(&mut self, request: Self::Request) -> Result<Self::Response, Self::Error> {
        let start = std::time::Instant::now();
        let message = request.to_string();

        let result = A::handle(self, request).await;

        match result {
            Ok(_) => {
                tracing::info!(user = %self.user.email(), latency = ?start.elapsed(), "{message}");
            }
            Err(ref error) if error.is_warn() => {
                tracing::warn!(%error, user = %self.user.email(), latency = ?start.elapsed(), "{message}");
            }
            Err(ref error) => {
                tracing::error!(%error, user = %self.user.email(), latency = ?start.elapsed(), "{message}");
            }
        }

        result
    }
}
