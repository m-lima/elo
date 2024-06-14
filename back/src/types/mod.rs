pub type Id = i64;

#[derive(Debug, Clone)]
pub enum User {
    Existing(ExistingUser),
    Pending(PendingUser),
}

#[derive(Debug, Clone)]
pub struct ExistingUser {
    pub id: i64,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct PendingUser {
    pub id: i64,
    pub email: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created: chrono::DateTime<chrono::Utc>,
    pub rating: f64,
}
