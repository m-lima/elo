pub type Id = i64;

// TODO: Move this to handler. Then, on auth, detach it from these types
#[derive(Debug, Clone)]
pub enum User {
    Existing(ExistingUser),
    Pending(PendingUser),
}

#[derive(Debug, Clone)]
pub struct ExistingUser {
    pub id: Id,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct PendingUser {
    pub id: Id,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: Id,
    pub name: String,
    pub email: String,
    pub inviter: Id,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created: chrono::DateTime<chrono::Utc>,
    pub rating: f64,
}
