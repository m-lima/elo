use super::Store;
use crate::types;

impl Store {
    pub async fn initialize(&self, rating: f64) -> Result<types::Player, sqlx::Error> {
        self.migrate().await?;

        sqlx::query_as!(
            types::Player,
            r#"
            INSERT INTO players (
                name,
                email,
                rating
            ) VALUES (
                $1,
                $2,
                $3
            ) RETURNING
                id,
                name,
                email,
                inviter,
                rating,
                created_ms AS "created_ms: types::Millis"
            "#,
            crate::consts::mock::USER_NAME,
            crate::consts::mock::USER_EMAIL,
            rating,
        )
        .fetch_one(&self.pool)
        .await
    }
}
