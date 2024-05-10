// ALLOW(clippy::missing_errors_doc): It is internal
#![allow(clippy::missing_errors_doc)]

mod error;
mod model;
mod store;

pub type Result<T = ()> = std::result::Result<T, error::Error>;

pub use error::Error;
pub use model::Id;
pub use store::Store;
