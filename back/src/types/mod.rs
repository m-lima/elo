pub type Id = i64;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub inviter: Option<Id>,
    pub rating: f64,
    pub wins: i64,
    pub losses: i64,
    pub points_won: i64,
    pub points_lost: i64,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayerTuple(
    pub Id,
    pub String,
    pub String,
    pub Option<Id>,
    pub f64,
    pub i64,
    pub i64,
    pub i64,
    pub i64,
    pub Millis,
);

impl From<Player> for PlayerTuple {
    fn from(value: Player) -> Self {
        Self(
            value.id,
            value.name,
            value.email,
            value.inviter,
            value.rating,
            value.wins,
            value.losses,
            value.points_won,
            value.points_lost,
            value.created_ms,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
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
            value.created_ms,
        )
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
