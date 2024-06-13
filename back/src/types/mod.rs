pub type Id = i64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created: chrono::DateTime<chrono::Utc>,
}
