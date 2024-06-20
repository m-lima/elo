#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to perform query: {0:?}")]
    Query(sqlx::Error),
    #[error("Attempted to store blank value for `{0}`")]
    BlankValue(&'static str),
    #[error("Entry already exists")]
    AlreadyExists,
    #[error("Conflicting values")]
    Conflict,
    #[error("Not found")]
    NotFound,
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(e) if e.code().map_or(false, |c| c == "2067") => {
                Error::AlreadyExists
            }
            sqlx::Error::RowNotFound => Error::NotFound,
            e => Error::Query(e),
        }
    }
}
