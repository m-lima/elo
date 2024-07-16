mod framework;

mod forbidden;
mod invite;
mod player;

use crate::{store, types};

const TESTER_NAME: &str = "tester";
const TESTER_EMAIL: &str = "tester@email.com";
const INVITED_NAME: &str = "invited";
const INVITED_EMAIL: &str = "invited@email.com";
const WHITE_SPACE: &str = " 	\n	 ";

async fn init(pool: &sqlx::SqlitePool) -> sqlx::Result<(types::Player, store::Store)> {
    let player = add_test_user(pool).await?;
    let store = store::Store::from(pool.clone());

    Ok((player, store))
}

async fn add_test_user(pool: &sqlx::sqlite::SqlitePool) -> sqlx::Result<types::Player> {
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
            1000
        ) RETURNING
            id,
            name,
            email,
            inviter,
            rating,
            created_ms AS "created_ms: types::Millis"
        "#,
        TESTER_NAME,
        TESTER_EMAIL,
    )
    .fetch_one(pool)
    .await
}
