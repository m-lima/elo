mod users;

#[derive(Debug, Clone)]
pub struct Store {
    pool: sqlx::sqlite::SqlitePool,
}

impl Store {
    pub async fn new<P>(path: P) -> Result<Self, sqlx::Error>
    where
        P: AsRef<std::path::Path>,
    {
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(path)
            .optimize_on_close(true, Some(1000))
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    #[must_use]
    pub fn users(&self) -> users::Users<'_> {
        users::Users::from(self)
    }
}
