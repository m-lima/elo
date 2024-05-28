#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not bind to address: {0:?}")]
    Bind(#[from] std::io::Error),
    #[error("Failed to create shutdown hook: {0:?}")]
    Shutdown(#[from] boile_rs::rt::shutdown::Error),
}
