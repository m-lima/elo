use super::super::{error::Error, model};
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
    pub async fn id_for(&self, email: &str) -> Result<Option<types::Id>> {
        let email = email.trim();
        if email.is_empty() {
            return Err(Error::BlankValue("email"));
        }

        sqlx::query_as!(
            model::Id,
            r#"
            SELECT
                id
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
        .map(|r| r.map(|id| id.id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<types::Player>> {
        sqlx::query_as!(
            model::Player,
            r#"
            SELECT
                id,
                name,
                email,
                created_ms AS "created_ms: model::Millis",
                rating
            FROM
                players
            "#
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Query)
        .map(|r| r.into_iter().map(types::Player::from).collect())
    }

    #[tracing::instrument(skip(self))]
    pub async fn rename(&self, id: types::Id, name: &str) -> Result<Option<types::Id>> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::BlankValue("name"));
        }

        sqlx::query_as!(
            model::Id,
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

    #[tracing::instrument(skip(self))]
    pub async fn invite(&self, inviter: types::Id, name: &str, email: &str) -> Result<types::Id> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::BlankValue("name"));
        }

        let email = email.trim();
        if email.is_empty() {
            return Err(Error::BlankValue("email"));
        }

        let mut tx = self.pool.begin().await.map_err(Error::Query)?;

        if sqlx::query_as!(
            model::Id,
            r#"
            SELECT
                id
            FROM
                players
            WHERE
                email = $1
            "#,
            email
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(Error::Query)?
        .is_some()
        {
            return Err(Error::AlreadyExists);
        }

        let id = sqlx::query_as!(
            model::Id,
            r#"
            INSERT INTO invites (
                inviter,
                name,
                email
            ) VALUES (
                $1,
                $2,
                $3
            ) RETURNING
                id
            "#,
            inviter,
            name,
            email
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::from)
        .map(|r| r.id)?;

        tx.commit().await.map_err(Error::Query)?;

        Ok(id)
    }
}
