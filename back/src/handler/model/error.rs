use crate::{mailbox, store, ws};

#[derive(Debug)]
pub enum Error {
    Store(store::Error),
    InvalidEmail(mailbox::Error),
    Forbidden,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Store(error) => error.fmt(f),
            Self::InvalidEmail(error) => error.fmt(f),
            Self::Forbidden => f.write_str("Forbidden"),
        }
    }
}

impl From<Error> for ws::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Store(error) => match error {
                store::Error::Query(_) => Self::new(
                    hyper::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                ),
                error @ (store::Error::BlankValue(_) | store::Error::InvalidValue(_)) => {
                    Self::new(hyper::StatusCode::BAD_REQUEST, &error)
                }
                error @ store::Error::AlreadyExists => {
                    Self::new(hyper::StatusCode::CONFLICT, &error)
                }
                store::Error::NotFound => Self::new(hyper::StatusCode::NOT_FOUND, "Not found"),
            },
            Error::InvalidEmail(error) => Self::new(hyper::StatusCode::BAD_REQUEST, &error),
            Error::Forbidden => Self::new(hyper::StatusCode::FORBIDDEN, "Forbidden"),
        }
    }
}

impl Error {
    pub fn is_warn(&self) -> bool {
        !matches!(self, Self::Store(store::Error::Query(_)))
    }
}
