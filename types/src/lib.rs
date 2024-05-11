pub type Id = i64;

pub struct User {
    pub id: i64,
    pub email: String,
    pub created: chrono::DateTime<chrono::Utc>,
}

pub struct Ranking {
    pub user: User,
    pub score: u32,
    pub wins: u16,
    pub losses: u16,
    pub points_won: u32,
    pub points_lost: u32,
}
