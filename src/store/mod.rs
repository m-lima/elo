mod error;
mod model;
mod store;

pub type Result<T = ()> = std::result::Result<T, error::Error>;

pub use error::Error;
pub use store::Store;
