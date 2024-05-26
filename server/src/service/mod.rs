mod message;

pub struct Service {
    store: store::Store,
    user: types::User,
    broadcaster: Broadcaster<message::Push>,
}

impl Service {
    pub fn new(store: store::Store, user: types::User) -> Self {
        let broadcaster = Broadcaster::new();

        Self {
            store,
            user,
            broadcaster,
        }
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<message::Push> {
        self.broadcaster.subscribe()
    }
}

#[derive(Debug)]
struct UserHandler {
    store: store::Store,
    user: types::User,
}

impl UserHandler {
    fn new(store: store::Store, user: types::User) -> Self {
        Self { store, user }
    }

    async fn handle(self, user: message::User) -> Result<message::Response, message::Error> {
        match user {
            message::User::Info => Ok(message::Response::User(self.user)),
            message::User::List => self
                .store
                .users()
                .list()
                .await
                .map(message::Response::Users)
                .map_err(|_| message::Error::Store),
            message::User::Get { email } => match self.store.users().get(&email).await {
                Ok(Some(user)) => Ok(message::Response::User(user)),
                Ok(None) => Err(message::Error::NotFound),
                Err(error) => {
                    tracing::error!(%error, "Could not fetch user");
                    Err(message::Error::Store)
                }
            },
        }
    }
}

impl tower_service::Service<message::Request> for Service {
    type Response = message::Response;
    type Error = message::Error;
    type Future = std::pin::Pin<
        Box<dyn Send + std::future::Future<Output = Result<Self::Response, Self::Error>>>,
    >;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: message::Request) -> Self::Future {
        match request {
            message::Request::User(user) => {
                Box::pin(UserHandler::new(self.store.clone(), self.user.clone()).handle(user))
            }
        }
    }
}

struct Broadcaster<T>
where
    T: Clone,
{
    sender: tokio::sync::broadcast::Sender<T>,
}

impl<T> Broadcaster<T>
where
    T: Clone,
{
    fn new() -> Self {
        let (sender, _) = tokio::sync::broadcast::channel(16);
        Self { sender }
    }

    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<T> {
        self.sender.subscribe()
    }

    fn send(&self, payload: T) {
        if let Ok(count) = self.sender.send(payload) {
            if count == 1 {
                tracing::debug!("Broadcasting to 1 listener");
            } else {
                tracing::debug!("Broadcasting to {count} listeners");
            }
        }
    }
}
