mod broadcaster;
mod handler;
mod message;

#[derive(Debug)]
pub struct Control {
    store: store::Store,
    user_id: types::Id,
    broadcaster: broadcaster::Broadcaster<message::Push>,
}

impl Control {
    #[must_use]
    pub fn new(store: store::Store, user_id: types::Id) -> Self {
        let broadcaster = broadcaster::Broadcaster::new();

        Self {
            store,
            user_id,
            broadcaster,
        }
    }
}

impl ws::Service for Control {
    type Request = message::Request;
    type Response = message::Response;
    type Error = message::Error;
    type Push = message::Push;

    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<Self::Push> {
        self.broadcaster.subscribe()
    }

    async fn call(&mut self, request: Self::Request) -> Result<Self::Response, Self::Error> {
        match request {
            message::Request::User(user) => handler::user(self).handle(user).await,
        }
    }
}
