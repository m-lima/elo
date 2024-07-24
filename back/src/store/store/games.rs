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
        Self::list_games(self.pool).await
    }

    #[tracing::instrument(skip(self, rating_updater))]
    pub async fn register<F>(
        &self,
        (player_one, player_two): (types::Id, types::Id),
        (score_one, score_two): (u8, u8),
        challenge: bool,
        millis: types::Millis,
        default_rating: f64,
        rating_updater: F,
    ) -> Result<(types::Id, Vec<types::Game>)>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> f64,
    {
        validate_game(player_one, player_two, score_one, score_two)?;

        let mut tx = self.pool.begin().await?;

        if challenge {
            validate_challenge(player_one, player_two, millis, None, tx.as_mut()).await?;
        }

        let game = sqlx::query_as!(
            super::Id,
            r#"
            INSERT INTO games (
                player_one,
                player_two,
                score_one,
                score_two,
                challenge,
                rating_one,
                rating_two,
                rating_delta,
                millis
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                0,
                0,
                0,
                $6
            )
            RETURNING
                id
            "#,
            player_one,
            player_two,
            score_one,
            score_two,
            challenge,
            millis,
        )
        .fetch_one(tx.as_mut())
        .await
        .map(|r| r.id)?;

        let updates = Self::execute_refresh(default_rating, rating_updater, &mut tx).await?;

        tx.commit().await?;

        Ok((game, updates))
    }

    #[tracing::instrument(skip(self, rating_updater))]
    pub async fn update<F>(
        &self,
        game: &types::Game,
        default_rating: f64,
        rating_updater: F,
    ) -> Result<(types::Id, Vec<types::Game>)>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> f64,
    {
        validate_game(
            game.player_one,
            game.player_two,
            game.score_one,
            game.score_two,
        )?;

        let mut tx = self.pool.begin().await?;

        if game.challenge {
            validate_challenge(
                game.player_one,
                game.player_two,
                game.millis,
                Some(game.id),
                tx.as_mut(),
            )
            .await?;
        }

        let game = sqlx::query_as!(
            super::Id,
            r#"
            UPDATE games
            SET
                player_one = $2,
                player_two = $3,
                score_one = $4,
                score_two = $5,
                challenge = $6,
                deleted = $7,
                millis = $8
            WHERE
                id = $1
            RETURNING
                id AS "id!: _"
            "#,
            game.id,
            game.player_one,
            game.player_two,
            game.score_one,
            game.score_two,
            game.challenge,
            game.deleted,
            game.millis,
        )
        .fetch_one(tx.as_mut())
        .await
        .map(|r| r.id)?;

        let updates = Self::execute_refresh(default_rating, rating_updater, &mut tx).await?;

        tx.commit().await?;

        Ok((game, updates))
    }

    async fn list_games<'c, 'e, E>(executor: E) -> Result<Vec<types::Game>>
    where
        'c: 'e,
        E: 'e + sqlx::Executor<'c, Database = sqlx::Sqlite>,
    {
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
                rating_delta,
                challenge,
                deleted,
                millis AS "millis: types::Millis",
                created_ms AS "created_ms: types::Millis"
            FROM
                games
            ORDER BY
                millis ASC
            "#
        )
        .fetch_all(executor)
        .await
        .map_err(Error::from)
    }

    #[tracing::instrument(skip(self, rating_updater))]
    pub async fn refresh<F>(
        &self,
        default_rating: f64,
        rating_updater: F,
    ) -> Result<Vec<types::Game>>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> f64,
    {
        let mut tx = self.pool.begin().await?;
        let games = Self::execute_refresh(default_rating, rating_updater, &mut tx).await?;
        tx.commit().await?;

        Ok(games)
    }

    async fn execute_refresh<F>(
        default_rating: f64,
        rating_updater: F,
        tx: &mut sqlx::Transaction<'static, sqlx::Sqlite>,
    ) -> Result<Vec<types::Game>>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> f64,
    {
        let updates = Self::build_updates(default_rating, rating_updater, tx.as_mut()).await?;

        if let Some(mut query) = build_update_query(&updates) {
            query
                .build_query_as()
                .fetch_all(tx.as_mut())
                .await
                .map_err(Into::into)
        } else {
            Ok(Vec::new())
        }
    }

    // TODO: Dont load the whole table. Get the latest rating for each player prior to edit,
    // defauting to `default_rating`, and rebuild from there:
    // with ratings as (select player_one,  player_two, rating_one, rating_two, max(millis) as millis from games group by player_one, player_two), unified as (select player_one as player, rating_one as rating, millis from ratings union select player_two as player, rating_two as rating, millis from ratings) select player, rating, max(millis) from unified group by player;
    // TODO: This would allow the front end to not have to fetch all games
    // TODO: This would also mean moving the EnrichedPlayer to the backend, so not all games need
    // to be loaded
    async fn build_updates<'c, 'e, E, F>(
        default_rating: f64,
        rating_updater: F,
        executor: E,
    ) -> Result<Vec<RatingUpdate>>
    where
        'c: 'e,
        E: 'e + sqlx::Executor<'c, Database = sqlx::Sqlite>,
        F: Copy + Fn(f64, f64, bool, bool) -> f64,
    {
        macro_rules! f64_ne {
            ($one: expr, $two: expr) => {
                ($one - $two).abs() > f64::EPSILON
            };
        }

        let updates = Self::list_games(executor).await?;
        let mut last_ratings = std::collections::HashMap::<types::Id, f64>::new();

        Ok(updates
            .into_iter()
            .filter_map(|game| {
                let rating_one = last_ratings
                    .get(&game.player_one)
                    .copied()
                    .unwrap_or(default_rating);
                let rating_two = last_ratings
                    .get(&game.player_two)
                    .copied()
                    .unwrap_or(default_rating);

                let rating_delta = if game.deleted {
                    0.0
                } else {
                    rating_updater(
                        rating_one,
                        rating_two,
                        game.score_one > game.score_two,
                        game.challenge,
                    )
                };

                last_ratings.insert(game.player_one, rating_one + rating_delta);
                last_ratings.insert(game.player_two, rating_two - rating_delta);

                (f64_ne!(rating_one, game.rating_one)
                    || f64_ne!(rating_two, game.rating_two)
                    || f64_ne!(rating_delta, game.rating_delta))
                .then_some(RatingUpdate {
                    id: game.id,
                    rating_one,
                    rating_two,
                    rating_delta,
                })
            })
            .collect())
    }
}

#[derive(Debug)]
struct RatingUpdate {
    id: types::Id,
    rating_one: f64,
    rating_two: f64,
    rating_delta: f64,
}

fn build_update_query(
    updates: &[RatingUpdate],
) -> Option<sqlx::QueryBuilder<'static, sqlx::Sqlite>> {
    if updates.is_empty() {
        return None;
    }

    let mut builder = sqlx::QueryBuilder::new("UPDATE games SET rating_one = CASE");
    for update in updates {
        builder.push(" WHEN id = ");
        builder.push_bind(update.id);
        builder.push(" THEN ");
        builder.push_bind(update.rating_one);
    }
    builder.push(" ELSE rating_one END, rating_two = CASE");
    for update in updates {
        builder.push(" WHEN id = ");
        builder.push_bind(update.id);
        builder.push(" THEN ");
        builder.push_bind(update.rating_two);
    }
    builder.push(" ELSE rating_two END, rating_delta = CASE");
    for update in updates {
        builder.push(" WHEN id = ");
        builder.push_bind(update.id);
        builder.push(" THEN ");
        builder.push_bind(update.rating_delta);
    }
    builder.push(" ELSE rating_delta END WHERE id IN (");
    let mut separated_builder = builder.separated(',');
    for update in updates {
        separated_builder.push_bind(update.id);
    }
    builder.push(") RETURNING id, player_one, player_two, score_one, score_two, rating_one, rating_two, rating_delta, challenge, deleted, millis, created_ms");

    Some(builder)
}

fn validate_game(
    player_one: types::Id,
    player_two: types::Id,
    score_one: impl Into<i64>,
    score_two: impl Into<i64>,
) -> Result {
    let score_one = score_one.into();
    let score_two = score_two.into();
    if player_one == player_two {
        Err(Error::InvalidValue("Players cannot be equal"))
    } else if score_one == score_two {
        Err(Error::InvalidValue("Scores cannot be equal"))
    } else if score_one > 12 || score_two > 12 {
        Err(Error::InvalidValue(
            "Games cannot have a score larger than 12",
        ))
    } else if score_one < 11 && score_two < 11 {
        Err(Error::InvalidValue(
            "Games must have a winner with at least 11 points",
        ))
    } else if (score_one == 12 && score_two != 10) || (score_two == 12 && score_one != 10) {
        Err(Error::InvalidValue("Tie breaks require a 12x10 score"))
    } else if (score_one == 11 && score_two >= 11) || (score_two == 11 && score_one >= 11) {
        Err(Error::InvalidValue("There can only be one winner"))
    } else {
        Ok(())
    }
}

async fn validate_challenge<'c, 'e, E>(
    player_one: types::Id,
    player_two: types::Id,
    millis: types::Millis,
    ignore: Option<types::Id>,
    executor: E,
) -> Result
where
    'c: 'e,
    E: 'e + sqlx::Executor<'c, Database = sqlx::Sqlite>,
{
    let millis = i64::from(millis);
    let challenged_today = if let Some(ignore) = ignore {
        sqlx::query!(
            r#"
            SELECT
                id
            FROM
                games
            WHERE
                challenge
                AND NOT deleted
                AND player_one IN ($1, $2)
                AND player_two IN ($1, $2)
                AND STRFTIME('%Y%m%d', $3 / 1000, 'unixepoch') = STRFTIME('%Y%m%d', millis / 1000, 'unixepoch')
                AND id <> $4
            "#,
            player_one,
            player_two,
            ignore,
            millis,
        )
        .fetch_optional(executor)
        .await?
        .is_some()
    } else {
        sqlx::query!(
            r#"
            SELECT
                id
            FROM
                games
            WHERE
                challenge
                AND NOT deleted
                AND player_one IN ($1, $2)
                AND player_two IN ($1, $2)
                AND STRFTIME('%Y%m%d', $3 / 1000, 'unixepoch') = STRFTIME('%Y%m%d', millis / 1000, 'unixepoch')
            "#,
            player_one,
            player_two,
            millis,
        )
        .fetch_optional(executor)
        .await?
        .is_some()
    };

    if challenged_today {
        return Err(Error::InvalidValue(
            "Players cannot challenge each other more than once a day",
        ));
    }

    Ok(())
}
