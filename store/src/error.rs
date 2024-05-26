#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to connect to the database: {0:?}")]
    Connection(sqlx::Error),
    #[error("Failed to perform query: {0:?}")]
    Query(sqlx::Error),
    #[error("Attempted to store blank value for `{0}`")]
    BlankValue(&'static str),
}
