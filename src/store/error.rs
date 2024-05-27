#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to connect to the database: {0:?}")]
    Connection(sqlx::Error),
    #[error("Failed to perform query: {0:?}")]
    Query(sqlx::Error),
    #[error("Failed to acquire a transaction: {0:?}")]
    Transaction(sqlx::Error),
    #[error("Attempted to store blank value for `{0}`")]
    BlankValue(&'static str),
    #[error("Entry already exists")]
    AlreadyExists,
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(e) => {
                if let Some(code) = e.code() {
                    if code.as_ref() == "2067" {
                        Error::AlreadyExists
                    } else {
                        Error::Query(sqlx::Error::Database(e))
                    }
                } else {
                    Error::Query(sqlx::Error::Database(e))
                }
            }
            e => Error::Query(e),
        }
    }
}
