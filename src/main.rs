mod args;

fn main() -> std::process::ExitCode {
    let args = args::parse();

    if let Err(error) = boile_rs::log::setup(boile_rs::log::Stdout, args.verbosity.level) {
        eprintln!("{error:?}");
        return std::process::ExitCode::FAILURE;
    }

    tracing::info!(
        verbosity = %args.verbosity.level,
        port = %args.port,
        "Configuration loaded"
    );

    if let Err(error) = boile_rs::rt::block_on(async_main(args)) {
        tracing::error!(?error, "Failed to start async environment");
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}

async fn async_main(args: args::Args) -> std::process::ExitCode {
    let router = axum::Router::new().route("/", axum::routing::get(|| async { "I'm alive!" }));
    let address = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), args.port);

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
