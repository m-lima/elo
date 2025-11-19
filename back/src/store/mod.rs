mod error;
mod store;

pub use error::Error;

pub type Store = store::Store<crate::rating::Elo>;
