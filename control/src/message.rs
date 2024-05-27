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
    List,
    Get(String),
    Invite(String),
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Push {
    Invited(String),
}

#[derive(Debug)]
pub enum Error {
    Store(store::Error),
    NotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Store(store) => store.fmt(f),
            Error::NotFound => f.write_str("Not found"),
        }
    }
}

impl ws::IntoError for Error {
    fn is_warn(&self) -> bool {
        match self {
            Error::Store(_) => false,
            Error::NotFound => true,
        }
    }
}

impl From<Error> for ws::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Store(_) => Self::from(hyper::StatusCode::INTERNAL_SERVER_ERROR),
            Error::NotFound => Self::from(hyper::StatusCode::NOT_FOUND),
        }
    }
}

impl ws::Request for Request {
    fn action(&self) -> &'static str {
        match self {
            Self::User(user) => match user {
                User::Info => "User::Info",
                User::List => "User::List",
                User::Get(_) => "User::Get",
                User::Invite(_) => "User::Invite",
            },
        }
    }
}
