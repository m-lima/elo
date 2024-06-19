use crate::{mailbox, store, types, ws};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Request {
    Player(Player),
    Invite(Invite),
    Game(Game),
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player(resource) => match resource {
                Player::Id => f.write_str("Player::Id"),
                Player::List => f.write_str("Player::List"),
                Player::Rename(_) => f.write_str("Player::Renmae"),
            },
            Self::Invite(resource) => match resource {
                Invite::Player(_) => f.write_str("Invite::Player"),
                Invite::Cancel(_) => f.write_str("Invite::Cancel"),
                Invite::Accept => f.write_str("Invite::Accept"),
                Invite::Reject => f.write_str("Invite::Reject"),
            },
            Self::Game(resource) => match resource {
                Game::List => f.write_str("Game::List"),
            },
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Player {
    Id,
    List,
    Rename(String),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Invite {
    Player(InvitePlayer),
    Cancel(types::Id),
    Accept,
    Reject,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct InvitePlayer {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Game {
    List,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Response {
    Id(types::Id),
    Players(Vec<types::PlayerTuple>),
    Games(Vec<types::GameTuple>),
    Done,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Push {
    Renamed(Renamed),
    Invited(InvitePlayer),
    Uninvited(types::Id),
    Joined(types::Player),
}

#[derive(Debug, Clone, serde::Serialize)]
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

impl std::error::Error for Error {}

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
