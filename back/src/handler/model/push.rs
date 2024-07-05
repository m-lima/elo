use crate::types;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Push {
    Player(Player),
    Game(Game),
}

impl std::fmt::Display for Push {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player(resource) => match resource {
                Player::Renamed { .. } => f.write_str("Player::Renamed"),
                Player::Invited { .. } => f.write_str("Player::Invited"),
                Player::Uninvited(_) => f.write_str("Player::Uninvited"),
                Player::Joined(_) => f.write_str("Player::Joined"),
            },
            Self::Game(resource) => match resource {
                Game::Registered(_, _, _) => f.write_str("Game::Registered"),
            },
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Player {
    Renamed {
        player: types::Id,
        old: String,
        new: String,
    },
    Invited(types::Invite),
    Uninvited(types::Invite),
    Joined(types::Player),
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Game {
    Registered(types::Game, types::Player, types::Player),
}
