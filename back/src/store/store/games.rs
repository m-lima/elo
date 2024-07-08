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
                challenge,
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

    #[allow(clippy::too_many_arguments)]
    #[tracing::instrument(skip(self, rating_updater))]
    pub async fn register<F>(
        &self,
        player_one: types::Id,
        player_two: types::Id,
        score_one: u8,
        score_two: u8,
        challenge: bool,
        default_rating: f64,
        rating_updater: F,
    ) -> Result<(types::Game, types::Player, types::Player)>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> (f64, f64),
    {
        if player_one == player_two {
            return Err(Error::InvalidValue("Players cannot be equal"));
        }

        if score_one == score_two {
            return Err(Error::InvalidValue("Scores cannot be equal"));
        }

        if (score_one == 12 && score_two != 10) || (score_two == 12 && score_one != 10) {
            return Err(Error::InvalidValue("Tie breaks require a 12x10 score"));
        }

        if (score_one == 11 && score_two >= 11) || (score_two == 11 && score_one >= 11) {
            return Err(Error::InvalidValue("There can only be one winner"));
        }

        let mut tx = self.pool.begin().await.map_err(Error::Query)?;

        let (rating_one, rating_two) = sqlx::query!(
            r#"
            SELECT
                one.rating as one,
                two.rating as two
            FROM
                (
                    SELECT
                        rating
                    from
                        players
                    where
                        id = $1
                ) as one,
                (
                    select
                        rating
                    from
                        players
                    where
                        id = $2
                ) as two;
            "#,
            player_one,
            player_two,
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(Error::Query)
        .map(|r| (r.one, r.two))?;

        let game = sqlx::query_as!(
            types::Game,
            r#"
            INSERT INTO games (
                player_one,
                player_two,
                score_one,
                score_two,
                rating_one,
                rating_two,
                challenge
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7
            )
            RETURNING
                id,
                player_one,
                player_two,
                score_one,
                score_two,
                rating_one,
                rating_two,
                challenge,
                created_ms AS "created_ms: types::Millis"
            "#,
            player_one,
            player_two,
            score_one,
            score_two,
            rating_one,
            rating_two,
            challenge,
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(Error::Query)?;

        let (rating_one, rating_two) =
            rating_updater(rating_one, rating_two, score_one > score_two, challenge);

        let (winner, loser, winner_score, loser_score, winner_rating, loser_rating) =
            if score_one > score_two {
                (
                    game.player_one,
                    game.player_two,
                    game.score_one,
                    game.score_two,
                    rating_one,
                    rating_two,
                )
            } else {
                (
                    game.player_two,
                    game.player_one,
                    game.score_two,
                    game.score_one,
                    rating_two,
                    rating_one,
                )
            };

        let (one, two) = {
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
                winner,
                winner_score,
                loser_score,
                winner_rating,
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
                loser,
                loser_score,
                winner_score,
                loser_rating,
            )
            .fetch_one(tx.as_mut())
            .await
            .map_err(Error::Query)?;

            (one, two)
        };

        tx.commit().await.map_err(Error::Query)?;

        Ok((game, one, two))
    }
}
