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

    #[tracing::instrument(skip(self, rating_updater))]
    pub async fn register<F>(
        &self,
        player_one: types::Id,
        player_two: types::Id,
        score_one: u8,
        score_two: u8,
        challenge: bool,
        rating_updater: F,
    ) -> Result<(types::Game, types::Player, types::Player)>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> (f64, f64),
    {
        validate_game(player_one, player_two, score_one, score_two)?;

        let mut tx = self.pool.begin().await.map_err(Error::Query)?;

        let (rating_one, rating_two) = ratings(player_one, player_two, tx.as_mut()).await?;

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

        let one = sqlx::query_as!(
            types::Player,
            r#"
            UPDATE
                players
            SET
                rating = $2
            WHERE
                id = $1
            RETURNING
                id AS "id!: _",
                name AS "name!: _",
                email AS "email!: _",
                inviter AS "inviter!: _",
                rating AS "rating!: _",
                created_ms AS "created_ms!: types::Millis"
            "#,
            game.player_one,
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
                rating = $2
            WHERE
                id = $1
            RETURNING
                id AS "id!: _",
                name AS "name!: _",
                email AS "email!: _",
                inviter AS "inviter!: _",
                rating AS "rating!: _",
                created_ms AS "created_ms!: types::Millis"
            "#,
            game.player_two,
            rating_two,
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(Error::Query)?;

        tx.commit().await.map_err(Error::Query)?;

        Ok((game, one, two))
    }
}

fn validate_game(
    player_one: types::Id,
    player_two: types::Id,
    score_one: u8,
    score_two: u8,
) -> Result {
    if player_one == player_two {
        Err(Error::InvalidValue("Players cannot be equal"))
    } else if score_one == score_two {
        Err(Error::InvalidValue("Scores cannot be equal"))
    } else if (score_one == 12 && score_two != 10) || (score_two == 12 && score_one != 10) {
        Err(Error::InvalidValue("Tie breaks require a 12x10 score"))
    } else if (score_one == 11 && score_two >= 11) || (score_two == 11 && score_one >= 11) {
        Err(Error::InvalidValue("There can only be one winner"))
    } else {
        Ok(())
    }
}

async fn ratings<'c, 'e, E>(
    player_one: types::Id,
    player_two: types::Id,
    executor: E,
) -> Result<(f64, f64)>
where
    'c: 'e,
    E: 'e + sqlx::Executor<'c, Database = sqlx::Sqlite>,
{
    sqlx::query!(
        r#"
        SELECT
            one.rating as one,
            two.rating as two
        FROM
            (
                SELECT
                    rating
                FROM
                    players
                WHERE
                    id = $1
            ) AS one,
            (
                SELECT
                    rating
                FROM
                    players
                WHERE
                    id = $2
            ) AS two;
        "#,
        player_one,
        player_two,
    )
    .fetch_one(executor)
    .await
    .map_err(Error::Query)
    .map(|r| (r.one, r.two))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn ratings(pool: sqlx::sqlite::SqlitePool) {
        let one = sqlx::query_as!(
            types::Player,
            r#"
            INSERT INTO players (
                name,
                email,
                rating
            ) VALUES (
                'one',
                'one',
                100
            ) RETURNING
                id,
                name,
                email,
                inviter,
                rating,
                created_ms AS "created_ms: types::Millis"
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let two = sqlx::query_as!(
            types::Player,
            r#"
            INSERT INTO players (
                name,
                email,
                rating
            ) VALUES (
                'two',
                'two',
                200
            ) RETURNING
                id,
                name,
                email,
                inviter,
                rating,
                created_ms AS "created_ms: types::Millis"
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let (rating_one, rating_two) = super::ratings(one.id, two.id, &pool).await.unwrap();

        assert!((rating_one - one.rating).abs() < f64::EPSILON);
        assert!((rating_two - two.rating).abs() < f64::EPSILON);
    }
}
