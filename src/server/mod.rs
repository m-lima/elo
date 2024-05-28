mod layer;

use crate::{handler, smtp, store, types, ws};

pub async fn start(port: u16, store: store::Store, smtp: smtp::Smtp) -> std::process::ExitCode {
    let router = route(store.clone(), smtp)
        .layer(layer::auth(store))
        .layer(layer::logger());

    let address = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), port);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let server = match boile_rs::rt::Shutdown::new() {
        Ok(shutdown) => axum::serve(listener, router).with_graceful_shutdown(shutdown),
        Err(error) => {
            tracing::error!(?error, "Failed to create shutdown hook");
            return std::process::ExitCode::FAILURE;
        }
    };

    let start = std::time::Instant::now();

    if let Err(error) = server.await {
        tracing::error!(?error, duration = ?start.elapsed(), "Server execution aborted");
        std::process::ExitCode::FAILURE
    } else {
        tracing::info!(duration = ?start.elapsed(), "Server gracefully shutdown");
        std::process::ExitCode::SUCCESS
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
