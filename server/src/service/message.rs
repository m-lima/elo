#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Request {
    User(User),
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Response {
    User(types::User),
    Users(Vec<types::User>),
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum User {
    Info,
    List,
    Get { email: String },
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Push {
    None,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    Store,
    NotFound,
}

impl From<Error> for crate::ws::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Store => Self::from(hyper::StatusCode::INTERNAL_SERVER_ERROR),
            Error::NotFound => Self::from(hyper::StatusCode::NOT_FOUND),
        }
    }
}
