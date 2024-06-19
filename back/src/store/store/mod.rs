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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Id {
    pub id: crate::types::Id,
}
