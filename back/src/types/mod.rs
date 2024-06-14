pub type Id = i64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
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
    pub inviter: Option<Id>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created: chrono::DateTime<chrono::Utc>,
    pub rating: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    pub id: Id,
    pub inviter: Id,
    pub name: String,
    pub email: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created: chrono::DateTime<chrono::Utc>,
}
