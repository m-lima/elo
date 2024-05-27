use super::{Error, Result, Store};
use crate::model;

pub struct Users<'a> {
    pool: &'a sqlx::sqlite::SqlitePool,
}

impl Users<'_> {
    #[tracing::instrument(target = "store::user", skip(self), err)]
    pub async fn list(&self) -> Result<Vec<types::User>> {
        sqlx::query_as!(
            model::User,
            r#"
            SELECT
                id,
                name,
                email,
                created_ms AS "created_ms: model::Millis"
            FROM
                users
            "#
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Query)
        .map(|r| r.into_iter().map(types::User::from).collect())
    }

    #[tracing::instrument(target = "store::user", skip(self), err)]
    pub async fn by_email(&self, email: &str) -> Result<Option<types::User>> {
        let email = email.trim();

        sqlx::query_as!(
            model::User,
            r#"
            SELECT
                id,
                name,
                email,
                created_ms AS "created_ms: model::Millis"
            FROM
                users
            WHERE
                email = $1
            "#,
            email
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Query)
        .map(|r| r.map(types::User::from))
    }

    #[tracing::instrument(target = "store::user", skip(self), err)]
    pub async fn by_id(&self, id: types::Id) -> Result<Option<types::User>> {
        sqlx::query_as!(
            model::User,
            r#"
            SELECT
                id,
                name,
                email,
                created_ms AS "created_ms: model::Millis"
            FROM
                users
            WHERE
                id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Query)
        .map(|r| r.map(types::User::from))
    }

    #[tracing::instrument(target = "store::user", skip(self), err)]
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
                users
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

    #[tracing::instrument(target = "store::user", skip(self), err)]
    pub async fn rename(&self, id: types::Id, name: &str) -> Result<Option<types::Id>> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::BlankValue("name"));
        }

        sqlx::query_as!(
            model::Id,
            r#"
            UPDATE
                users
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
        .map_err(Error::Query)
        .map(|r| r.map(|id| id.id))
    }

    #[tracing::instrument(target = "store::user", skip(self), err)]
    pub async fn invite(&self, inviter: types::Id, name: &str, email: &str) -> Result<types::Id> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::BlankValue("name"));
        }

        let email = email.trim();
        if email.is_empty() {
            return Err(Error::BlankValue("email"));
        }

        let mut tx = self.pool.begin().await.map_err(Error::Transaction)?;

        if sqlx::query_as!(
            model::Id,
            r#"
            SELECT
                id
            FROM
                users
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
        .map_err(Error::Query)
        .map(|r| r.id)?;

        tx.commit().await.map_err(Error::Query)?;

        Ok(id)
    }
}

impl<'a> From<&'a Store> for Users<'a> {
    fn from(value: &'a Store) -> Self {
        Self { pool: &value.pool }
    }
}
