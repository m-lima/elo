use super::Store;
use crate::types;

impl Store {
    pub async fn migrate(
        &self,
        rating: f64,
        deviation: f64,
        volatility: f64,
    ) -> Result<types::Player, sqlx::Error> {
        sqlx::migrate!().run(&self.pool).await?;

        sqlx::query_as!(
            types::Player,
            r#"
            INSERT INTO players (
                name,
                email,
                rating,
                deviation,
                volatility
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            ) RETURNING
                id,
                name,
                email,
                inviter,
                created_ms AS "created_ms: types::Millis",
                rating
            "#,
            crate::consts::mock::USER_NAME,
            crate::consts::mock::USER_EMAIL,
            rating,
            deviation,
            volatility,
        )
        .fetch_one(&self.pool)
        .await
    }
}
