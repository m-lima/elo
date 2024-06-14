use super::super::{error::Error, model};
use crate::types;

type Result<T = ()> = std::result::Result<T, Error>;

pub struct Invites<'a> {
    pool: &'a sqlx::sqlite::SqlitePool,
}

impl<'a> From<&'a super::Store> for Invites<'a> {
    fn from(value: &'a super::Store) -> Self {
        Self { pool: &value.pool }
    }
}

impl Invites<'_> {
    pub async fn auth(&self, email: &str) -> Result<Option<types::User>> {
        let email = email.trim();
        if email.is_empty() {
            return Err(Error::BlankValue("email"));
        }

        sqlx::query_as!(
            model::User,
            r#"
            SELECT
                id,
                name,
                email
            FROM
                invites
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
        .fetch_optional(tx.as_mut())
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
        .fetch_one(tx.as_mut())
        .await
        .map_err(Error::Query)
        .map(|r| r.id)?;

        tx.commit().await.map_err(Error::Query)?;

        Ok(id)
    }

    #[tracing::instrument(skip(self))]
    pub async fn cancel(&self, inviter: types::Id, id: types::Id) -> Result<Option<types::Id>> {
        sqlx::query_as!(
            model::Id,
            r#"
            DELETE FROM
                invites
            WHERE
                id = $1 AND
                inviter = $2
            RETURNING
                id
            "#,
            id,
            inviter
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Query)
        .map(|r| r.map(|id| id.id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn accept(
        &self,
        id: types::Id,
        rating: f64,
        deviation: f64,
        volatility: f64,
    ) -> Result<Option<types::Player>> {
        let mut tx = self.pool.begin().await.map_err(Error::Query)?;

        let Some(invite) = sqlx::query_as!(
            model::Invite,
            r#"
            DELETE FROM
                invites
            WHERE
                id = $1
            RETURNING
                id,
                inviter,
                name,
                email,
                created_ms AS "created_ms: model::Millis"
            "#,
            id
        )
        .fetch_optional(tx.as_mut())
        .await
        .map_err(Error::Query)?
        else {
            return Ok(None);
        };

        let player = sqlx::query_as!(
            model::Player,
            r#"
            INSERT INTO players (
                name,
                email,
                inviter,
                rating,
                deviation,
                volatility
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            ) RETURNING
                id,
                name,
                email,
                inviter,
                created_ms AS "created_ms: model::Millis",
                rating
            "#,
            invite.name,
            invite.email,
            invite.inviter,
            rating,
            deviation,
            volatility
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(Error::Query)
        .map(types::Player::from)?;

        tx.commit().await.map_err(Error::Query)?;

        Ok(Some(player))
    }

    #[tracing::instrument(skip(self))]
    pub async fn reject(&self, id: types::Id) -> Result<Option<types::Invite>> {
        sqlx::query_as!(
            model::Invite,
            r#"
            DELETE FROM
                invites
            WHERE
                id = $1
            RETURNING
                id,
                inviter,
                name,
                email,
                created_ms AS "created_ms: model::Millis"
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Query)
        .map(|r| r.map(types::Invite::from))
    }
}
