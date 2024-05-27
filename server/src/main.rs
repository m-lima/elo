mod args;
mod layer;

#[allow(clippy::declare_interior_mutable_const)]
const X_USER: hyper::header::HeaderName = hyper::header::HeaderName::from_static("x-user");

fn setup_tracing(
    verbosity: args::Verbosity,
) -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    use tracing_subscriber::layer::SubscriberExt;

    let subscriber = tracing_subscriber::registry()
        .with(boile_rs::log::tracing::layer(boile_rs::log::Stdout))
        .with(tracing::level_filters::LevelFilter::from_level(
            verbosity.level,
        ));

    if verbosity.internal {
        tracing::subscriber::set_global_default(subscriber)
    } else {
        let subscriber = subscriber
            .with(tracing_subscriber::filter::Targets::new().with_target("elo", verbosity.level));
        tracing::subscriber::set_global_default(subscriber)
    }
}

fn main() -> std::process::ExitCode {
    let args = args::parse();

    if let Err(error) = setup_tracing(args.verbosity) {
        eprintln!("{error:?}");
        return std::process::ExitCode::FAILURE;
    }

    tracing::info!(target: "elo", verbosity = %args.verbosity.level, port = %args.port, "Configuration loaded");

    if let Err(error) = boile_rs::rt::block_on(async_main(args)) {
        tracing::error!(target: "elo", ?error, "Failed to start async environment");
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}

async fn async_main(args: args::Args) -> std::process::ExitCode {
    let store = match store::Store::new(&args.db).await {
        Ok(store) => store,
        Err(error) => {
            tracing::error!(target: "elo", ?error, db = ?args.db, "Failed to open store");
            return std::process::ExitCode::FAILURE;
        }
    };

    // let service = match smtp::Smtp::new(
    //     String::new(),
    //     String::new(),
    //     0,
    //     String::new(),
    //     String::new(),
    //     String::new(),
    // ) {
    //     Ok(service) => service,
    //     Err(error) => {
    //         tracing::error!(target: "elo", ?error, "Failed to create SMTP service");
    //         return std::process::ExitCode::FAILURE;
    //     }
    // };
    let service = smtp::Smtp::empty();

    let router = route(store.clone(), service)
        .layer(layer::auth(store))
        .layer(layer::logger());

    let address = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), args.port);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let server = match boile_rs::rt::Shutdown::new() {
        Ok(shutdown) => axum::serve(listener, router).with_graceful_shutdown(shutdown),
        Err(error) => {
            tracing::error!(target: "elo", ?error, "Failed to create shutdown hook");
            return std::process::ExitCode::FAILURE;
        }
    };

    let start = std::time::Instant::now();

    if let Err(error) = server.await {
        tracing::error!(target: "elo", ?error, duration = ?start.elapsed(), "Server execution aborted");
        std::process::ExitCode::FAILURE
    } else {
        tracing::info!(target: "elo", duration = ?start.elapsed(), "Server gracefully shutdown");
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
                    let control = control::Control::new(store, user.id, smtp);
                    let socket = ws::Layer::<M, _>::new(socket, control, user.email);
                    socket.serve()
                })
            },
        )
    }

    axum::Router::new()
        .route("/ws/text", upgrade::<String>(store.clone(), smtp.clone()))
        .route("/ws/binary", upgrade::<Vec<u8>>(store, smtp))
}
