mod invite;
mod player;

use super::{broadcaster, model};
use crate::{smtp, store, types, ws};

#[derive(Debug)]
pub struct Handler {
    user_id: types::Id,
    store: store::Store,
    smtp: smtp::Smtp,
    broadcaster: broadcaster::Broadcaster<model::Push>,
}

impl Handler {
    #[must_use]
    pub fn new(user_id: types::Id, store: store::Store, smtp: smtp::Smtp) -> Self {
        let broadcaster = broadcaster::Broadcaster::new();

        Self {
            user_id,
            store,
            smtp,
            broadcaster,
        }
    }
}

impl ws::Service for Handler {
    type Request = model::Request;
    type Response = model::Response;
    type Error = model::Error;
    type Push = model::Push;

    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<Self::Push> {
        self.broadcaster.subscribe()
    }

    async fn call(&mut self, request: Self::Request) -> Result<Self::Response, Self::Error> {
        match request {
            model::Request::Player(request) => player::Player::new(self).handle(request).await,
            model::Request::Invite(request) => invite::Invite::new(self).handle(request).await,
        }
    }
}
