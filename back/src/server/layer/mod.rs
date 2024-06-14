pub mod auth;
mod logger;

pub fn logger() -> logger::Logger {
    logger::Logger
}
