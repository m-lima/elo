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
                store::Error::Query(_) => Self::from(hyper::StatusCode::INTERNAL_SERVER_ERROR),
                store::Error::BlankValue(error) => {
                    Self::new(hyper::StatusCode::BAD_REQUEST, error.to_string())
                }
                store::Error::AlreadyExists | store::Error::Conflict => {
                    Self::from(hyper::StatusCode::CONFLICT)
                }
                store::Error::NotFound => Self::from(hyper::StatusCode::NOT_FOUND),
            },
            Error::InvalidEmail(error) => {
                Self::new(hyper::StatusCode::BAD_REQUEST, error.to_string())
            }
            Error::Forbidden => Self::from(hyper::StatusCode::FORBIDDEN),
        }
    }
}

impl Error {
    pub fn is_warn(&self) -> bool {
        match self {
            Self::Store(store::Error::Query(_)) => false,
            Self::Store(
                store::Error::BlankValue(_)
                | store::Error::AlreadyExists
                | store::Error::Conflict
                | store::Error::NotFound,
            )
            | Self::InvalidEmail(_)
            | Self::Forbidden => true,
        }
    }
}
