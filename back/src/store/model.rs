use crate::types;

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Id {
    pub id: types::Id,
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub(crate) struct User {
    pub id: types::Id,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub(crate) struct Player {
    pub id: types::Id,
    pub name: String,
    pub email: String,
    pub inviter: Option<types::Id>,
    pub created_ms: Millis,
    pub rating: f64,
}

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub(crate) struct Invite {
    pub id: types::Id,
    pub inviter: types::Id,
    pub email: String,
    pub name: String,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub(crate) struct Match {
    pub id: types::Id,
    pub player_one: types::Id,
    pub player_two: types::Id,
    pub score_one: i64,
    pub score_two: i64,
    pub accepted: bool,
    pub created_ms: Millis,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
#[repr(transparent)]
#[sqlx(transparent)]
pub(crate) struct Millis(i64);

impl From<Millis> for chrono::DateTime<chrono::Utc> {
    fn from(value: Millis) -> Self {
        chrono::DateTime::from_timestamp(value.0, 0).unwrap()
    }
}

impl From<User> for types::User {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}

impl From<Player> for types::Player {
    fn from(value: Player) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            inviter: value.inviter,
            created: value.created_ms.into(),
            rating: value.rating,
        }
    }
}

impl From<Invite> for types::Invite {
    fn from(value: Invite) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            inviter: value.inviter,
            created: value.created_ms.into(),
        }
    }
}
