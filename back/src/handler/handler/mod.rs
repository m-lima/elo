mod game;
mod invite;
mod player;

use super::{access, broadcaster, model};
use crate::{smtp, store, ws};

pub trait Access: access::Access + Sized {
    fn handle<S>(
        handler: &mut Handler<Self, S>,
        request: model::Request,
    ) -> impl std::future::Future<Output = Result<model::Response, model::Error>>
    where
        S: smtp::Smtp;
}

macro_rules! impl_access {
    ($type: ty) => {
        impl Access for $type {
            async fn handle<S>(
                handler: &mut Handler<Self, S>,
                request: model::Request,
            ) -> Result<model::Response, model::Error>
            where
                S: smtp::Smtp,
            {
                match request {
                    model::Request::Player(request) => {
                        player::Player::new(handler).handle(request).await
                    }
                    model::Request::Invite(request) => {
                        invite::Invite::new(handler).handle(request).await
                    }
                    model::Request::Game(request) => game::Game::new(handler).handle(request).await,
                }
            }
        }
    };
}

impl_access!(access::Regular);
impl_access!(access::Pending);

#[derive(Debug)]
pub struct Handler<A, S>
where
    A: Access,
    S: smtp::Smtp,
{
    user: access::User<A>,
    store: store::Store,
    smtp: S,
    broadcaster: broadcaster::Broadcaster,
}

impl<A, S> Handler<A, S>
where
    A: Access,
    S: smtp::Smtp,
{
    #[must_use]
    pub fn new(
        user: access::User<A>,
        store: store::Store,
        broadcaster: broadcaster::Broadcaster,
        smtp: S,
    ) -> Self {
        Self {
            user,
            store,
            smtp,
            broadcaster,
        }
    }
}

impl<A, S> ws::Service for Handler<A, S>
where
    A: Access,
    S: smtp::Smtp,
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
