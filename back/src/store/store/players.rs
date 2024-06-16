use super::super::error::Error;
use crate::types;

type Result<T = ()> = std::result::Result<T, Error>;

pub struct Players<'a> {
    pool: &'a sqlx::sqlite::SqlitePool,
}

impl<'a> From<&'a super::Store> for Players<'a> {
    fn from(value: &'a super::Store) -> Self {
        Self { pool: &value.pool }
    }
}

impl Players<'_> {
    #[tracing::instrument(skip(self))]
    pub async fn auth(&self, email: &str) -> Result<Option<types::User>> {
        let email = email.trim();
        if email.is_empty() {
            return Err(Error::BlankValue("email"));
        }

        sqlx::query_as!(
            types::User,
            r#"
            SELECT
                id,
                name,
                email
            FROM
                players
            WHERE
                email = $1
            "#,
            email
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Query)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get(&self, id: types::Id) -> Result<Option<types::Player>> {
        sqlx::query_as!(
            types::Player,
            r#"
            SELECT
                id,
                name,
                email,
                inviter,
                created_ms AS "created_ms: types::Millis",
                rating
            FROM
                players
            ORDER BY
                rating DESC,
                created_ms ASC
            "#
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Query)
    }

    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<types::Player>> {
        sqlx::query_as!(
            types::Player,
            r#"
            SELECT
                id,
                name,
                email,
                inviter,
                created_ms AS "created_ms: types::Millis",
                rating
            FROM
                players
            ORDER BY
                rating DESC,
                created_ms ASC
            "#
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Query)
    }

    #[tracing::instrument(skip(self))]
    pub async fn rename(&self, id: types::Id, name: &str) -> Result<Option<types::Id>> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::BlankValue("name"));
        }

        sqlx::query_as!(
            super::Id,
            r#"
            UPDATE
                players
            SET
                name = $2
            WHERE
                id = $1
            RETURNING
                id AS "id!: _"
            "#,
            id,
            name
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::from)
        .map(|r| r.map(|id| id.id))
    }
}
