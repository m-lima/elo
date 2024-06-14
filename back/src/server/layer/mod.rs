mod auth;
mod logger;

use crate::handler;

pub fn auth(handler: handler::Auth) -> auth::Auth {
    auth::Auth::new(handler)
}

pub fn logger() -> logger::Logger {
    logger::Logger
}
