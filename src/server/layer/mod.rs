mod auth;
mod logger;

use crate::store;

pub fn auth(store: store::Store) -> auth::Auth {
    auth::Auth::new(store)
}

pub fn logger() -> logger::Logger {
    logger::Logger
}
