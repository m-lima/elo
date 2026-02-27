use super::error::Error;
use super::layer;
use crate::{args, handler, smtp, store, ws};

pub async fn serve<S>(
    socket: args::Socket,
    store: store::Store,
    broadcaster: handler::Broadcaster,
    smtp: S,
) -> Result<(), Error>
where
    S: smtp::Smtp,
{
    let router = route(store.clone(), broadcaster, smtp)
        .layer(layer::auth::Auth::new(handler::Auth::new(store.clone())))
        .layer(layer::logger());

    #[cfg(feature = "local")]
    let router = router.layer(tower_http::cors::CorsLayer::very_permissive());

    let shutdown = boile_rs::rt::Shutdown::new()?;

    match socket {
        args::Socket::Port(port) => {
            let address = std::net::SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, port);
            let listener = tokio::net::TcpListener::bind(address).await?;
            inner_serve(listener, router, shutdown).await
        }
        args::Socket::Unix(path) => {
            let listener = tokio::net::UnixListener::bind(path)?;
            inner_serve(listener, router, shutdown).await
        }
    }
}

async fn inner_serve<L>(
    listener: L,
    router: axum::Router,
    shutdown: boile_rs::rt::Shutdown,
) -> Result<(), Error>
where
    L: axum::serve::Listener,
    axum::serve::WithGracefulShutdown<L, axum::Router, axum::Router, boile_rs::rt::Shutdown>:
        std::future::IntoFuture<Output = std::io::Result<()>>,
{
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(Error::Run)
}

fn route<S>(store: store::Store, broadcaster: handler::Broadcaster, smtp: S) -> axum::Router
where
    S: smtp::Smtp,
{
    fn upgrade<M: ws::Mode, S: smtp::Smtp>(
        store: store::Store,
        broadcaster: handler::Broadcaster,
        smtp: S,
    ) -> axum::routing::MethodRouter<()> {
        axum::routing::get(
            |upgrade: axum::extract::WebSocketUpgrade,
             axum::Extension(user): axum::Extension<handler::UserAccess>| async {
                upgrade.on_upgrade(|socket| async {
                    macro_rules! serve {
                        ($user: expr) => {{
                            let handler = handler::Handler::new($user, store, broadcaster, smtp);
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
        .route(
            "/ws/text",
            upgrade::<String, S>(store.clone(), broadcaster.clone(), smtp.clone()),
        )
        .route(
            "/ws/binary",
            upgrade::<Vec<u8>, S>(store, broadcaster, smtp),
        )
}
