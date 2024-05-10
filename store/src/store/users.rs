use super::{Error, Result, Store};
use crate::model;

pub struct Users<'a> {
    pool: &'a sqlx::sqlite::SqlitePool,
}

impl Users<'_> {
    #[tracing::instrument(skip(self), err)]
    pub async fn list(&self) -> Result<Vec<model::User>> {
        sqlx::query_as!(
            model::User,
            r#"
            SELECT
                id,
                email,
                created_ms AS "created_ms: model::Millis"
            FROM
                users
            "#
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Query)
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get(&self, email: &str) -> Result<model::User> {
        let email = email.trim();

        sqlx::query_as!(
            model::User,
            r#"
            SELECT
                id,
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
        .and_then(|r| r.ok_or(Error::NotFound))
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn create(&self, email: &str) -> Result<model::User> {
        let email = email.trim();
        if email.is_empty() {
            return Err(Error::BlankValue("email"));
        }

        sqlx::query_as!(
            model::User,
            r#"
            INSERT INTO users (
                email
            ) VALUES (
                $1
            ) RETURNING
                id,
                email,
                created_ms AS "created_ms: model::Millis"
            "#,
            email
        )
        .fetch_one(self.pool)
        .await
        .map_err(Error::Query)
    }
}

impl<'a> From<&'a Store> for Users<'a> {
    fn from(value: &'a Store) -> Self {
        Self { pool: &value.pool }
    }
}
