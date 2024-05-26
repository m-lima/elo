pub type Id = i64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ranking {
    pub user: User,
    pub score: u32,
    pub wins: u16,
    pub losses: u16,
    pub points_won: u32,
    pub points_lost: u32,
}
