use crate::{mailbox, store, types, ws};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Request {
    User(User),
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Response {
    Id(types::Id),
    User(types::User),
    Users(Vec<types::User>),
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum User {
    Info,
    Rename(String),
    List,
    Get(String),
    Invite(Invite),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Push {
    Invited(Invite),
}

#[derive(Debug)]
pub enum Error {
    Store(store::Error),
    NotFound,
    InvalidEmail(mailbox::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Store(error) => error.fmt(f),
            Error::NotFound => f.write_str("Not found"),
            Error::InvalidEmail(error) => error.fmt(f),
        }
    }
}

impl ws::IntoError for Error {
    fn is_warn(&self) -> bool {
        match self {
            Error::Store(error) => match error {
                store::Error::Query(_) => true,
                store::Error::BlankValue(_) | store::Error::AlreadyExists => false,
            },
            Error::NotFound | Error::InvalidEmail(_) => true,
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
                store::Error::AlreadyExists => Self::from(hyper::StatusCode::CONFLICT),
            },
            Error::NotFound => Self::from(hyper::StatusCode::NOT_FOUND),
            Error::InvalidEmail(error) => {
                Self::new(hyper::StatusCode::BAD_REQUEST, error.to_string())
            }
        }
    }
}

impl ws::Request for Request {
    fn action(&self) -> &'static str {
        match self {
            Self::User(user) => match user {
                User::Info => "User::Info",
                User::Rename(_) => "User::Renmae",
                User::List => "User::List",
                User::Get(_) => "User::Get",
                User::Invite(_) => "User::Invite",
            },
        }
    }
}
