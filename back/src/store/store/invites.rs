use super::super::error::Error;
use crate::types;

type Result<T = ()> = std::result::Result<T, Error>;

pub struct Invites<'a> {
    store: &'a super::Store,
}

impl<'a> From<&'a super::Store> for Invites<'a> {
    fn from(store: &'a super::Store) -> Self {
        Self { store }
    }
}

impl Invites<'_> {
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
                invites
            WHERE
                email = $1
            "#,
            email
        )
        .fetch_optional(&self.store.pool)
        .await
        .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<types::Invite>> {
        sqlx::query_as!(
            types::Invite,
            r#"
            SELECT
                id,
                inviter,
                name,
                email,
                created_ms AS "created_ms: types::Millis"
            FROM
                invites
            "#
        )
        .fetch_all(&self.store.pool)
        .await
        .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    pub async fn invite(
        &self,
        inviter: types::Id,
        name: &str,
        email: &str,
    ) -> Result<types::Invite> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::BlankValue("name"));
        }

        let email = email.trim().to_lowercase();
        if email.is_empty() {
            return Err(Error::BlankValue("email"));
        }

        let mut tx = self.store.pool.begin().await?;

        if sqlx::query_as!(
            super::Id,
            r#"
            SELECT
                id
            FROM
                players
            WHERE
                name = $1 OR
                email = $2
            "#,
            name,
            email,
        )
        .fetch_optional(tx.as_mut())
        .await?
        .is_some()
        {
            return Err(Error::AlreadyExists);
        }

        let invite = sqlx::query_as!(
            types::Invite,
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
                id,
                inviter,
                name,
                email,
                created_ms AS "created_ms: types::Millis"
            "#,
            inviter,
            name,
            email
        )
        .fetch_one(tx.as_mut())
        .await?;

        tx.commit().await?;

        self.store.update_version();

        Ok(invite)
    }

    #[tracing::instrument(skip(self))]
    pub async fn cancel(&self, inviter: types::Id, id: types::Id) -> Result<types::Invite> {
        sqlx::query_as!(
            types::Invite,
            r#"
            DELETE FROM
                invites
            WHERE
                id = $1 AND
                inviter = $2
            RETURNING
                id,
                inviter,
                name,
                email,
                created_ms AS "created_ms: types::Millis"
            "#,
            id,
            inviter
        )
        .fetch_one(&self.store.pool)
        .await
        .inspect(|_| self.store.update_version())
        .map_err(Error::from)
    }

    #[tracing::instrument(skip(self))]
    pub async fn accept(&self, id: types::Id) -> Result<(types::Player, types::User)> {
        let mut tx = self.store.pool.begin().await?;

        let invite = sqlx::query_as!(
            types::Invite,
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
                created_ms AS "created_ms: types::Millis"
            "#,
            id,
        )
        .fetch_one(tx.as_mut())
        .await?;

        let player = sqlx::query_as!(
            types::Player,
            r#"
            INSERT INTO players (
                name,
                email,
                inviter
            ) VALUES (
                $1,
                $2,
                $3
            ) RETURNING
                id,
                name,
                email,
                inviter,
                created_ms AS "created_ms: types::Millis"
            "#,
            invite.name,
            invite.email,
            invite.inviter,
        )
        .fetch_one(tx.as_mut())
        .await?;

        let inviter = sqlx::query_as!(
            types::User,
            r#"
            SELECT
                id,
                name,
                email
            FROM
                players
            WHERE
                id = $1
            "#,
            invite.inviter,
        )
        .fetch_one(tx.as_mut())
        .await?;

        tx.commit().await?;

        self.store.update_version();

        Ok((player, inviter))
    }

    #[tracing::instrument(skip(self))]
    pub async fn reject(&self, id: types::Id) -> Result<(types::Invite, types::User)> {
        let mut tx = self.store.pool.begin().await?;

        let invite = sqlx::query_as!(
            types::Invite,
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
                created_ms AS "created_ms: types::Millis"
            "#,
            id,
        )
        .fetch_one(tx.as_mut())
        .await?;

        let inviter = sqlx::query_as!(
            types::User,
            r#"
            SELECT
                id,
                name,
                email
            FROM
                players
            WHERE
                id = $1
            "#,
            invite.inviter,
        )
        .fetch_one(tx.as_mut())
        .await?;

        tx.commit().await?;

        self.store.update_version();

        Ok((invite, inviter))
    }
}
