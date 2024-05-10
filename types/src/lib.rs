pub struct User<Id> {
    pub id: Id,
    pub email: String,
    pub created: chrono::DateTime<chrono::Utc>,
}

pub struct Ranking<Id> {
    pub user: User<Id>,
    pub score: u32,
    pub wins: u16,
    pub losses: u16,
    pub points_won: u32,
    pub points_lost: u32,
}
