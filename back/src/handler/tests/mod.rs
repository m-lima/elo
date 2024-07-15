use super::*;
use crate::{server, smtp, store, types, ws};

mod forbidden;
mod invite;

const TESTER_NAME: &str = "tester";
const TESTER_EMAIL: &str = "tester@email.com";

async fn init(pool: &sqlx::SqlitePool) -> (types::Player, store::Store) {
    let player = add_test_user(pool).await;
    let store = store::Store::from(pool.clone());

    (player, store)
}

async fn add_test_user(pool: &sqlx::sqlite::SqlitePool) -> types::Player {
    sqlx::query_as!(
        types::Player,
        r#"
        INSERT INTO players (
            name,
            email,
            rating
        ) VALUES (
            $1,
            $2,
            1000
        ) RETURNING
            id,
            name,
            email,
            inviter,
            rating,
            created_ms AS "created_ms: types::Millis"
        "#,
        TESTER_NAME,
        TESTER_EMAIL,
    )
    .fetch_one(pool)
    .await
    .unwrap()
}

#[derive(Clone)]
struct TestSmtp {
    tx: tokio::sync::mpsc::Sender<smtp::Payload>,
}

impl TestSmtp {
    fn new() -> (Self, tokio::sync::mpsc::Receiver<smtp::Payload>) {
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        (Self { tx }, rx)
    }
}

impl smtp::Smtp for TestSmtp {
    async fn send(&mut self, payload: smtp::Payload) {
        self.tx.send(payload).await.unwrap();
    }
}

struct RichHandler<A>
where
    A: handler::Access,
{
    handler: Handler<A, TestSmtp>,
    push: tokio::sync::broadcast::Receiver<model::Push>,
    email: tokio::sync::mpsc::Receiver<smtp::Payload>,
}

impl RichHandler<access::Regular> {
    async fn new(user: &str, store: &store::Store) -> Self {
        let broadcaster = Broadcaster::new();
        let push = broadcaster.subscribe();
        let (smtp, email) = TestSmtp::new();

        let auth = access::Auth::new(store.clone());
        let user = match server::auth::Provider::auth(&auth, user)
            .await
            .unwrap()
            .unwrap()
        {
            access::Dynamic::Regular(user) => user,
            access::Dynamic::Pending(_) => unreachable!(),
        };

        let handler = handler::Handler::new(user, store.clone(), broadcaster, smtp);

        Self {
            handler,
            push,
            email,
        }
    }
}

impl RichHandler<access::Pending> {
    async fn pending(user: &str, store: &store::Store) -> Self {
        let broadcaster = Broadcaster::new();
        let push = broadcaster.subscribe();
        let (smtp, email) = TestSmtp::new();

        let auth = access::Auth::new(store.clone());
        let user = match server::auth::Provider::auth(&auth, user)
            .await
            .unwrap()
            .unwrap()
        {
            access::Dynamic::Pending(user) => user,
            access::Dynamic::Regular(_) => unreachable!(),
        };

        let handler = handler::Handler::new(user, store.clone(), broadcaster, smtp);

        Self {
            handler,
            push,
            email,
        }
    }
}

impl<A> RichHandler<A>
where
    A: handler::Access,
{
    async fn call_ok(&mut self, request: model::Request) -> model::Response {
        use ws::Service;
        self.handler.call(request).await.unwrap()
    }

    async fn call_err(&mut self, request: model::Request) -> model::Error {
        use ws::Service;
        let result = self.handler.call(request).await.unwrap_err();
        self.check_no_message();
        result
    }

    fn check_no_message(&mut self) {
        assert_eq!(
            self.push.try_recv().unwrap_err(),
            tokio::sync::broadcast::error::TryRecvError::Empty
        );

        assert_eq!(
            self.email.try_recv().unwrap_err(),
            tokio::sync::mpsc::error::TryRecvError::Empty
        );
    }
}
