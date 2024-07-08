use crate::types;

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
                Invite::List => f.write_str("Invite::List"),
                Invite::Player { .. } => f.write_str("Invite::Player"),
                Invite::Cancel(_) => f.write_str("Invite::Cancel"),
                Invite::Accept => f.write_str("Invite::Accept"),
                Invite::Reject => f.write_str("Invite::Reject"),
            },
            Self::Game(resource) => match resource {
                Game::List => f.write_str("Game::List"),
                Game::Register { .. } => f.write_str("Game::Register"),
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

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Invite {
    List,
    Player { name: String, email: String },
    Cancel(types::Id),
    Accept,
    Reject,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Game {
    List,
    #[serde(rename_all = "camelCase")]
    Register {
        opponent: types::Id,
        score: u8,
        opponent_score: u8,
        challenge: bool,
    },
}
