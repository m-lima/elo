pub type Id = i64;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub inviter: Option<Id>,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayerTuple(pub Id, pub String, pub String, pub Option<Id>, pub Millis);

impl From<Player> for PlayerTuple {
    fn from(value: Player) -> Self {
        Self(
            value.id,
            value.name,
            value.email,
            value.inviter,
            value.created_ms,
        )
    }
}

impl From<PlayerTuple> for Player {
    fn from(value: PlayerTuple) -> Self {
        Self {
            id: value.0,
            name: value.1,
            email: value.2,
            inviter: value.3,
            created_ms: value.4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub id: Id,
    pub inviter: Id,
    pub name: String,
    pub email: String,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteTuple(pub Id, pub Id, pub String, pub String, pub Millis);

impl From<Invite> for InviteTuple {
    fn from(value: Invite) -> Self {
        Self(
            value.id,
            value.inviter,
            value.name,
            value.email,
            value.created_ms,
        )
    }
}

impl From<InviteTuple> for Invite {
    fn from(value: InviteTuple) -> Self {
        Self {
            id: value.0,
            inviter: value.1,
            name: value.2,
            email: value.3,
            created_ms: value.4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Game {
    pub id: Id,
    pub player_one: Id,
    pub player_two: Id,
    pub score_one: i64,
    pub score_two: i64,
    pub rating_one: f64,
    pub rating_two: f64,
    pub rating_delta: f64,
    pub challenge: bool,
    pub deleted: bool,
    pub millis: Millis,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct GameTuple(
    pub Id,
    pub Id,
    pub Id,
    pub i64,
    pub i64,
    pub f64,
    pub f64,
    pub f64,
    pub bool,
    pub bool,
    pub Millis,
    pub Millis,
);

impl From<Game> for GameTuple {
    fn from(value: Game) -> Self {
        Self(
            value.id,
            value.player_one,
            value.player_two,
            value.score_one,
            value.score_two,
            value.rating_one,
            value.rating_two,
            value.rating_delta,
            value.challenge,
            value.deleted,
            value.millis,
            value.created_ms,
        )
    }
}

impl From<GameTuple> for Game {
    fn from(value: GameTuple) -> Self {
        Self {
            id: value.0,
            player_one: value.1,
            player_two: value.2,
            score_one: value.3,
            score_two: value.4,
            rating_one: value.5,
            rating_two: value.6,
            rating_delta: value.7,
            challenge: value.8,
            deleted: value.9,
            millis: value.10,
            created_ms: value.11,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct History {
    pub id: Id,
    pub game: Id,
    pub player_one: Id,
    pub player_two: Id,
    pub score_one: i64,
    pub score_two: i64,
    pub challenge: bool,
    pub deleted: bool,
    pub millis: Millis,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct HistoryTuple(
    pub Id,
    pub Id,
    pub Id,
    pub Id,
    pub i64,
    pub i64,
    pub bool,
    pub bool,
    pub Millis,
    pub Millis,
);

impl From<History> for HistoryTuple {
    fn from(value: History) -> Self {
        Self(
            value.id,
            value.game,
            value.player_one,
            value.player_two,
            value.score_one,
            value.score_two,
            value.challenge,
            value.deleted,
            value.millis,
            value.created_ms,
        )
    }
}

impl From<HistoryTuple> for History {
    fn from(value: HistoryTuple) -> Self {
        Self {
            id: value.0,
            game: value.1,
            player_one: value.2,
            player_two: value.3,
            score_one: value.4,
            score_two: value.5,
            challenge: value.6,
            deleted: value.7,
            millis: value.8,
            created_ms: value.9,
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[repr(transparent)]
#[sqlx(transparent)]
pub(crate) struct Millis(i64);

impl From<i64> for Millis {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<Millis> for i64 {
    fn from(value: Millis) -> Self {
        value.0
    }
}
