use super::error::Error;
use super::layer;
use crate::{handler, smtp, store, ws};

pub struct Server {
    server: axum::serve::WithGracefulShutdown<axum::Router, axum::Router, boile_rs::rt::Shutdown>,
}

impl Server {
    pub async fn new(port: u16, store: store::Store, smtp: smtp::Smtp) -> Result<Self, Error> {
        let router = route(store.clone(), smtp)
            .layer(layer::auth::Auth::new(handler::Auth::new(store.clone())))
            .layer(layer::logger());

        #[cfg(feature = "local")]
        let router = router.layer(tower_http::cors::CorsLayer::very_permissive());

        let address = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), port);
        let listener = tokio::net::TcpListener::bind(address).await?;
        let shutdown = boile_rs::rt::Shutdown::new()?;
        let server = axum::serve(listener, router).with_graceful_shutdown(shutdown);

        Ok(Self { server })
    }

    pub async fn start(self) -> std::io::Result<()> {
        self.server.await
    }
}

fn route(store: store::Store, smtp: smtp::Smtp) -> axum::Router {
    fn upgrade<M: ws::Mode>(
        store: store::Store,
        smtp: smtp::Smtp,
    ) -> axum::routing::MethodRouter<()> {
        axum::routing::get(
            |upgrade: axum::extract::WebSocketUpgrade,
             axum::Extension(user): axum::Extension<handler::UserAccess>| async {
                upgrade.on_upgrade(|socket| async {
                    macro_rules! serve {
                        ($user: expr) => {{
                            let handler = handler::Handler::new($user, store, smtp);
                            let socket = ws::Layer::<M, _>::new(socket, handler);
                            socket.serve().await;
                        }};
                    }

                    match user {
                        handler::UserAccess::Regular(user) => serve!(user),
                        handler::UserAccess::Pending(user) => serve!(user),
                    }
                })
            },
        )
    }

    axum::Router::new()
        .route("/ws/text", upgrade::<String>(store.clone(), smtp.clone()))
        .route("/ws/binary", upgrade::<Vec<u8>>(store, smtp))
}
