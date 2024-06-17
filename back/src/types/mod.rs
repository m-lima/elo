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
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayerTuple(Id, String, String, Option<Id>, f64, Millis);

impl From<Player> for PlayerTuple {
    fn from(value: Player) -> Self {
        Self(
            value.id,
            value.name,
            value.email,
            value.inviter,
            value.rating,
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
    pub accepted: bool,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct GameTuple(Id, Id, Id, i64, i64, f64, f64, bool, Millis);

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
            value.accepted,
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
