use super::Store;
use crate::{rating, types};

impl<R> Store<R>
where
    R: rating::Config,
{
    pub async fn initialize(&self) -> Result<types::Player, sqlx::Error> {
        self.migrate().await?;

        sqlx::query_as!(
            types::Player,
            r#"
            INSERT INTO players (
                name,
                email
            ) VALUES (
                $1,
                $2
            ) RETURNING
                id,
                name,
                email,
                inviter,
                created_ms AS "created_ms: types::Millis"
            "#,
            crate::consts::mock::USER_NAME,
            crate::consts::mock::USER_EMAIL,
        )
        .fetch_one(&self.pool)
        .await
    }
}
