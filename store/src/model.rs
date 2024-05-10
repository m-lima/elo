pub type Id = i64;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct User {
    pub id: Id,
    pub email: String,
    pub created_ms: Millis,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Ranking {
    pub user: Id,
    pub email: String,
    pub created_ms: Millis,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
#[repr(transparent)]
#[sqlx(transparent)]
pub struct Millis(i64);
