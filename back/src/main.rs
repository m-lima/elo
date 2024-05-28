mod args;
mod handler;
mod mailbox;
mod server;
mod smtp;
mod store;
mod types;
mod ws;

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
        let subscriber = subscriber.with(
            tracing_subscriber::filter::Targets::new()
                .with_target(env!("CARGO_CRATE_NAME"), verbosity.level),
        );
        tracing::subscriber::set_global_default(subscriber)
    }
}

fn main() -> std::process::ExitCode {
    let args = args::parse();

    if let Err(error) = setup_tracing(args.verbosity) {
        eprintln!("{error:?}");
        return std::process::ExitCode::FAILURE;
    }

    tracing::info!(verbosity = %args.verbosity.level, port = %args.port, "Configuration loaded");

    if let Err(error) = boile_rs::rt::block_on(async_main(args)) {
        tracing::error!(?error, "Failed to start async environment");
        std::process::ExitCode::FAILURE
    } else {
        std::process::ExitCode::SUCCESS
    }
}

async fn async_main(args: args::Args) -> std::process::ExitCode {
    let store = match store::Store::new(&args.db).await {
        Ok(store) => store,
        Err(error) => {
            tracing::error!(?error, db = ?args.db, "Failed to open store");
            return std::process::ExitCode::FAILURE;
        }
    };

    let smtp = if let Some(smtp) = args.smtp {
        match smtp::Smtp::new(smtp.link, smtp.smtp, smtp.from).await {
            Ok(smtp) => smtp,
            Err(error) => {
                tracing::error!(?error, "Failed to create SMTP service");
                return std::process::ExitCode::FAILURE;
            }
        }
    } else {
        smtp::Smtp::empty()
    };

    let server = match server::Server::new(args.port, store, smtp).await {
        Ok(server) => server,
        Err(error) => {
            tracing::error!(?error, "Failed to create server");
            return std::process::ExitCode::FAILURE;
        }
    };

    let start = std::time::Instant::now();

    if let Err(error) = server.start().await {
        tracing::error!(?error, duration = ?start.elapsed(), "Server execution aborted");
        std::process::ExitCode::FAILURE
    } else {
        tracing::info!(duration = ?start.elapsed(), "Server gracefully shutdown");
        std::process::ExitCode::SUCCESS
    }
}
