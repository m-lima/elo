#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub(crate) struct User {
    pub id: types::Id,
    pub email: String,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub(crate) struct Ranking {
    pub user: types::Id,
    pub email: String,
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
            email: value.email,
            created: value.created_ms.into(),
        }
    }
}
