use super::super::error::Error;
use crate::types;

type Result<T = ()> = std::result::Result<T, Error>;

// TODO: Whenever `games` is modified (created, accepted, deleted), recalculate all ratings
// TODO: The rating on the game should be the incoming rating
// TODO: We get the rating for a match by getting the latest match and applying the rating
// calculator; falling back to a default value if a previous match does not exist

pub struct Games<'a> {
    pool: &'a sqlx::sqlite::SqlitePool,
}

impl<'a> From<&'a super::Store> for Games<'a> {
    fn from(value: &'a super::Store) -> Self {
        Self { pool: &value.pool }
    }
}

impl Games<'_> {
    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<types::Game>> {
        sqlx::query_as!(
            types::Game,
            r#"
            SELECT
                id,
                player_one,
                player_two,
                score_one,
                score_two,
                rating_one,
                rating_two,
                created_ms AS "created_ms: types::Millis"
            FROM
                games
            ORDER BY
                created_ms DESC
            "#
        )
        .fetch_all(self.pool)
        .await
        .map_err(Error::Query)
    }

    #[tracing::instrument(skip(self, rating_updater))]
    pub async fn register<F>(
        &self,
        player_one: types::Id,
        player_two: types::Id,
        score_one: u8,
        score_two: u8,
        default_rating: f64,
        rating_updater: F,
    ) -> Result<(types::Game, types::Player, types::Player)>
    where
        F: Copy + Fn(f64, f64, bool) -> (f64, f64),
    {
        if player_one == player_two || score_one == score_two {
            return Err(Error::Conflict);
        }

        let mut tx = self.pool.begin().await.map_err(Error::Query)?;

        let rating_one =
            Self::get_rating(player_one, default_rating, rating_updater, &mut tx).await?;
        let rating_two =
            Self::get_rating(player_two, default_rating, rating_updater, &mut tx).await?;

        let game = sqlx::query_as!(
            types::Game,
            r#"
            INSERT INTO games (
                player_one,
                player_two,
                score_one,
                score_two,
                rating_one,
                rating_two
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            )
            RETURNING
                id,
                player_one,
                player_two,
                score_one,
                score_two,
                rating_one,
                rating_two,
                created_ms AS "created_ms: types::Millis"
            "#,
            player_one,
            player_two,
            score_one,
            score_two,
            rating_one,
            rating_two,
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(Error::Query)?;

        let (rating_one, rating_two) =
            rating_updater(rating_one, rating_two, score_one > score_two);

        let (one, two) = if score_one > score_two {
            let one = sqlx::query_as!(
                types::Player,
                r#"
                UPDATE
                    players
                SET
                    wins = wins + 1,
                    points_won = points_won + $2,
                    points_lost = points_lost + $3,
                    rating = $4
                WHERE
                    id = $1
                RETURNING
                    id AS "id!: _",
                    name AS "name!: _",
                    email AS "email!: _",
                    inviter AS "inviter!: _",
                    rating AS "rating!: _",
                    wins AS "wins!: _",
                    losses AS "losses!: _",
                    points_won AS "points_won!: _",
                    points_lost AS "points_lost!: _",
                    created_ms AS "created_ms!: types::Millis"
                "#,
                game.player_one,
                game.score_one,
                game.score_two,
                rating_one,
            )
            .fetch_one(tx.as_mut())
            .await
            .map_err(Error::Query)?;

            let two = sqlx::query_as!(
                types::Player,
                r#"
                UPDATE
                    players
                SET
                    losses = losses + 1,
                    points_won = points_won + $2,
                    points_lost = points_lost + $3,
                    rating = $4
                WHERE
                    id = $1
                RETURNING
                    id AS "id!: _",
                    name AS "name!: _",
                    email AS "email!: _",
                    inviter AS "inviter!: _",
                    rating AS "rating!: _",
                    wins AS "wins!: _",
                    losses AS "losses!: _",
                    points_won AS "points_won!: _",
                    points_lost AS "points_lost!: _",
                    created_ms AS "created_ms!: types::Millis"
                "#,
                game.player_two,
                game.score_two,
                game.score_one,
                rating_two,
            )
            .fetch_one(tx.as_mut())
            .await
            .map_err(Error::Query)?;

            (one, two)
        } else {
            let one = sqlx::query_as!(
                types::Player,
                r#"
                UPDATE
                    players
                SET
                    losses = losses + 1,
                    points_won = points_won + $2,
                    points_lost = points_lost + $3,
                    rating = $4
                WHERE
                    id = $1
                RETURNING
                    id AS "id!: _",
                    name AS "name!: _",
                    email AS "email!: _",
                    inviter AS "inviter!: _",
                    rating AS "rating!: _",
                    wins AS "wins!: _",
                    losses AS "losses!: _",
                    points_won AS "points_won!: _",
                    points_lost AS "points_lost!: _",
                    created_ms AS "created_ms!: types::Millis"
                "#,
                game.player_one,
                game.score_one,
                game.score_two,
                rating_one,
            )
            .fetch_one(tx.as_mut())
            .await
            .map_err(Error::Query)?;

            let two = sqlx::query_as!(
                types::Player,
                r#"
                UPDATE
                    players
                SET
                    wins = wins + 1,
                    points_won = points_won + $2,
                    points_lost = points_lost + $3,
                    rating = $4
                WHERE
                    id = $1
                RETURNING
                    id AS "id!: _",
                    name AS "name!: _",
                    email AS "email!: _",
                    inviter AS "inviter!: _",
                    rating AS "rating!: _",
                    wins AS "wins!: _",
                    losses AS "losses!: _",
                    points_won AS "points_won!: _",
                    points_lost AS "points_lost!: _",
                    created_ms AS "created_ms!: types::Millis"
                "#,
                game.player_two,
                game.score_two,
                game.score_one,
                rating_two,
            )
            .fetch_one(tx.as_mut())
            .await
            .map_err(Error::Query)?;

            (one, two)
        };

        tx.commit().await.map_err(Error::Query)?;

        Ok((game, one, two))
    }

    async fn get_rating<'a, F>(
        player: types::Id,
        default_rating: f64,
        rating_updater: F,
        tx: &mut sqlx::Transaction<'a, sqlx::Sqlite>,
    ) -> Result<f64>
    where
        F: Fn(f64, f64, bool) -> (f64, f64),
    {
        sqlx::query!(
            r#"
            SELECT
                rating,
                opponent_rating,
                score,
                opponent_score
            FROM
            (
                SELECT
                    id,
                    rating_one as rating,
                    rating_two as opponent_rating,
                    score_one as score,
                    score_two as opponent_score,
                    created_ms
                FROM
                    games
                WHERE
                    player_two = $1
                UNION SELECT
                    id,
                    rating_two as rating,
                    rating_one as opponent_rating,
                    score_two as score,
                    score_one as opponent_score,
                    created_ms
                FROM
                    games
                WHERE
                    player_two = $1
            )
            ORDER BY
                created_ms DESC,
                id DESC
            LIMIT
                1
            "#,
            player,
        )
        .fetch_optional(tx.as_mut())
        .await
        .map(|r| {
            r.map_or(default_rating, |r| {
                rating_updater(r.rating, r.opponent_rating, r.score > r.opponent_score).0
            })
        })
        .map_err(Into::into)
    }
}
