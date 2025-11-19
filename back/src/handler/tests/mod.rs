mod framework;

macro_rules! init {
    ($pool: ident, $conn: ident) => {{
        let options = $conn.journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
        let pool = $pool
            .max_connections(1)
            .connect_with(options)
            .await
            .unwrap();
        let (player, store) = crate::handler::tests::init(&pool).await.unwrap();
        let handler = crate::handler::tests::framework::Handler::new(&player.email, &store)
            .await
            .unwrap();
        (player, store, handler, pool)
    }};
}

mod game;
mod invite;
mod player;

use crate::{store, types};

const TESTER_NAME: &str = "tester";
const TESTER_EMAIL: &str = "tester@email.com";
const INVITED_NAME: &str = "invited";
const INVITED_EMAIL: &str = "invited@email.com";
const ACCEPTED_NAME: &str = "accepted";
const ACCEPTED_EMAIL: &str = "accepted@email.com";
const WHITE_SPACE: &str = " 	\n	 ";
const DEFAULT_RATING: f64 = <crate::rating::Elo as crate::rating::Config>::DEFAULT_VALUE;

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
        TESTER_NAME,
        TESTER_EMAIL,
    )
    .fetch_one(pool)
    .await
}

fn default_rating_delta() -> f64 {
    skillratings::elo::elo(
        &skillratings::elo::EloRating::new(),
        &skillratings::elo::EloRating::new(),
        &skillratings::Outcomes::WIN,
        &skillratings::elo::EloConfig::new(),
    )
    .0
    .rating
        - skillratings::elo::EloRating::new().rating
}
