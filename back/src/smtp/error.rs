#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to build SMTP sender: {0:?}")]
    Transport(#[from] lettre::transport::smtp::Error),
    #[error("Failed to build SMTP sender: Could not connect")]
    Connection,
}
