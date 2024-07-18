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
        let email = email.trim().to_lowercase();
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
        .map_err(Error::from)
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
                created_ms AS "created_ms: types::Millis"
            FROM
                players
            ORDER BY
                created_ms ASC
            "#
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    pub async fn rename(&self, id: types::Id, name: &str) -> Result<types::Player> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::BlankValue("name"));
        }

        let mut tx = self.pool.begin().await?;

        if sqlx::query_as!(
            super::Id,
            r#"
            SELECT
                id
            FROM
                players
            WHERE
                name = $1
            UNION
            SELECT
                id
            FROM
                invites
            WHERE
                name = $1
            "#,
            name
        )
        .fetch_optional(tx.as_mut())
        .await?
        .is_some()
        {
            return Err(Error::AlreadyExists);
        }

        let player = sqlx::query_as!(
            types::Player,
            r#"
            UPDATE
                players
            SET
                name = $2
            WHERE
                id = $1
            RETURNING
                id AS "id!: _",
                name AS "name!: _",
                email AS "email!: _",
                inviter AS "inviter!: _",
                created_ms AS "created_ms!: types::Millis"
            "#,
            id,
            name
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(Error::from)?;

        tx.commit().await?;

        Ok(player)
    }
}
