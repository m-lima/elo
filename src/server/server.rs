use super::error::Error;
use super::layer;
use crate::{handler, smtp, store, types, ws};

pub struct Server {
    server: axum::serve::WithGracefulShutdown<axum::Router, axum::Router, boile_rs::rt::Shutdown>,
}

impl Server {
    pub async fn new(port: u16, store: store::Store, smtp: smtp::Smtp) -> Result<Self, Error> {
        let router = route(store.clone(), smtp)
            .layer(layer::auth(store))
            .layer(layer::logger());

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
             axum::Extension(user): axum::Extension<types::User>| async move {
                upgrade.on_upgrade(move |socket| {
                    let handler = handler::Handler::new(user.id, store, smtp);
                    let socket = ws::Layer::<M, _>::new(socket, handler, user.email);
                    socket.serve()
                })
            },
        )
    }

    axum::Router::new()
        .route("/ws/text", upgrade::<String>(store.clone(), smtp.clone()))
        .route("/ws/binary", upgrade::<Vec<u8>>(store, smtp))
}
