use crate::{mailbox, store, types, ws};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Request {
    Player(Player),
    Invite(Invite),
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player(player) => match player {
                Player::Id => f.write_str("Player::Id"),
                Player::List => f.write_str("Player::List"),
                Player::Rename(_) => f.write_str("Player::Renmae"),
            },
            Self::Invite(invite) => match invite {
                Invite::Player(_) => f.write_str("Invite::Player"),
                Invite::Cancel(_) => f.write_str("Invite::Cancel"),
                Invite::Accept => f.write_str("Invite::Accept"),
                Invite::Reject => f.write_str("Invite::Reject"),
            },
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Response {
    Id(types::Id),
    Players(Vec<types::Player>),
    Renamed,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Player {
    Id,
    List,
    Rename(String),
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Invite {
    Player(InvitePlayer),
    Cancel(types::Id),
    Accept,
    Reject,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InvitePlayer {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Push {
    Renamed(Renamed),
    Invited(InvitePlayer),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Renamed {
    pub player: types::Id,
    pub name: String,
}

#[derive(Debug)]
pub enum Error {
    Store(store::Error),
    NotFound,
    InvalidEmail(mailbox::Error),
    Forbidden,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Store(error) => error.fmt(f),
            Self::NotFound => f.write_str("Not found"),
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
                store::Error::AlreadyExists => Self::from(hyper::StatusCode::CONFLICT),
            },
            Error::NotFound => Self::from(hyper::StatusCode::NOT_FOUND),
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
            Self::Store(store::Error::BlankValue(_) | store::Error::AlreadyExists)
            | Self::NotFound
            | Self::InvalidEmail(_)
            | Self::Forbidden => true,
        }
    }
}
