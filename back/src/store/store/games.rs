use super::super::error::Error;
use crate::types;

type Result<T = ()> = std::result::Result<T, Error>;

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
                accepted,
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
        rating_updater: F,
    ) -> Result<Option<types::Game>>
    where
        F: Fn(f64, f64) -> (f64, f64),
    {
        if player_one == player_two || score_one == score_two {
            return Err(Error::Conflict);
        }

        let mut tx = self.pool.begin().await.map_err(Error::Query)?;

        let players = sqlx::query_as!(
            types::Player,
            r#"
            SELECT
                id,
                name,
                email,
                inviter,
                rating,
                wins,
                losses,
                points_won,
                points_lost,
                created_ms AS "created_ms: types::Millis"
            FROM
                players
            WHERE
                id IN ($1, $2)
            "#,
            player_one,
            player_two,
        )
        .fetch_all(tx.as_mut())
        .await
        .map_err(Error::Query)?;

        if players.len() != 2 {
            return Ok(None);
        }

        let (player_one, player_two) = {
            let mut players = players.into_iter();
            let (Some(one), Some(two)) = (players.next(), players.next()) else {
                return Ok(None);
            };

            if one.id == player_one {
                if two.id == player_two {
                    (one, two)
                } else {
                    return Ok(None);
                }
            } else if two.id == player_one {
                (two, one)
            } else {
                return Ok(None);
            }
        };

        let (rating_one, rating_two) = rating_updater(player_one.rating, player_two.rating);

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
                accepted,
                created_ms AS "created_ms: types::Millis"
            "#,
            player_one.id,
            player_two.id,
            score_one,
            score_two,
            rating_one,
            rating_two,
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(Error::Query)?;

        tx.commit().await.map_err(Error::Query)?;

        Ok(Some(game))
    }

    #[tracing::instrument(skip(self))]
    pub async fn accept(
        &self,
        player: types::Id,
        id: types::Id,
    ) -> Result<Option<(types::Id, types::Player, types::Player)>> {
        let mut tx = self.pool.begin().await.map_err(Error::Query)?;

        let Some(game) = sqlx::query_as!(
            types::Game,
            r#"
            UPDATE
                games
            SET
                accepted = true
            WHERE
                id = $1 AND
                player_two = $2
            RETURNING
                id AS "id!: _",
                player_one AS "player_one!: _",
                player_two AS "player_two!: _",
                score_one AS "score_one!: _",
                score_two AS "score_two!: _",
                rating_one AS "rating_one!: _",
                rating_two AS "rating_two!: _",
                accepted AS "accepted!: _",
                created_ms AS "created_ms!: types::Millis"
            "#,
            id,
            player,
        )
        .fetch_optional(tx.as_mut())
        .await
        .map_err(Error::Query)?
        else {
            tx.commit().await.map_err(Error::Query)?;
            return Ok(None);
        };

        let (one, two) = if game.score_one > game.score_two {
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
                game.rating_one,
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
                game.rating_two,
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
                game.rating_one,
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
                game.rating_two,
            )
            .fetch_one(tx.as_mut())
            .await
            .map_err(Error::Query)?;

            (one, two)
        };

        tx.commit().await.map_err(Error::Query)?;

        Ok(Some((game.id, one, two)))
    }

    #[tracing::instrument(skip(self))]
    pub async fn cancel(&self, player: types::Id, id: types::Id) -> Result<Option<types::Id>> {
        sqlx::query_as!(
            super::Id,
            r#"
            DELETE FROM
                games
            WHERE
                id = $1 AND
                player_one = $2
            RETURNING
                id
            "#,
            id,
            player,
        )
        .fetch_optional(self.pool)
        .await
        .map_err(Error::Query)
        .map(|r| r.map(|id| id.id))
    }
}
