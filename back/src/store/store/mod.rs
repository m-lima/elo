mod games;
mod invites;
mod players;

#[cfg(feature = "local")]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Store {
    pool: sqlx::sqlite::SqlitePool,
    version: std::sync::Arc<std::sync::atomic::AtomicU32>,
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

        let version = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(rand::random()));

        Ok(Self { pool, version })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!().run(&self.pool).await
    }

    #[must_use]
    pub fn version(&self) -> u32 {
        self.version.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[must_use]
    pub fn invites(&self) -> invites::Invites<'_> {
        invites::Invites::from(self)
    }

    #[must_use]
    pub fn games(&self) -> games::Games<'_> {
        games::Games::from(self)
    }

    #[must_use]
    pub fn players(&self) -> players::Players<'_> {
        players::Players::from(self)
    }

    fn update_version(&self) {
        self.version
            .store(rand::random(), std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
impl From<sqlx::SqlitePool> for Store {
    fn from(pool: sqlx::SqlitePool) -> Self {
        let version = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(rand::random()));
        Self { pool, version }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Id {
    pub id: crate::types::Id,
}
