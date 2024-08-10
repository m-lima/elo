use super::super::error::Error;
use crate::types;

type Result<T = ()> = std::result::Result<T, Error>;

pub struct Games<'a> {
    store: &'a super::Store,
}

impl<'a> From<&'a super::Store> for Games<'a> {
    fn from(store: &'a super::Store) -> Self {
        Self { store }
    }
}

impl Games<'_> {
    // TODO: This would allow the front end to not have to fetch all games
    // TODO: This would also mean moving the EnrichedPlayer to the backend, so not all games need
    // to be loaded
    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<types::Game>> {
        Self::list_games(&self.store.pool).await
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

        let mut tx = self.store.pool.begin().await?;

        if challenge {
            Self::validate_challenge(player_one, player_two, millis, None, tx.as_mut()).await?;
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

        let updates =
            Self::execute_refresh(Some(millis), default_rating, rating_updater, &mut tx).await?;

        tx.commit().await?;

        self.store.update_version();

        Ok((game, updates))
    }

    #[tracing::instrument(skip(self, rating_updater))]
    pub async fn update<F>(
        &self,
        game: &types::Game,
        default_rating: f64,
        rating_updater: F,
    ) -> Result<Vec<types::Game>>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> f64,
    {
        validate_game(
            game.player_one,
            game.player_two,
            game.score_one,
            game.score_two,
        )?;

        let mut tx = self.store.pool.begin().await?;

        if game.challenge && !game.deleted {
            Self::validate_challenge(
                game.player_one,
                game.player_two,
                game.millis,
                Some(game.id),
                tx.as_mut(),
            )
            .await?;
        }

        let old_millis = sqlx::query_scalar!(
            r#"
            SELECT
                millis AS "millis: types::Millis"
            FROM
                games
            WHERE
                id = $1
            "#,
            game.id,
        )
        .fetch_one(tx.as_mut())
        .await?;

        let new_millis = sqlx::query_scalar!(
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
                millis AS "millis!: types::Millis"
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
        .await?;

        let updates = Self::execute_refresh(
            Some(old_millis.min(new_millis)),
            default_rating,
            rating_updater,
            &mut tx,
        )
        .await?;

        tx.commit().await?;

        self.store.update_version();

        Ok(updates)
    }

    #[tracing::instrument(skip(self))]
    pub async fn history(&self, game: types::Id) -> Result<Vec<types::History>> {
        sqlx::query_as!(
            types::History,
            r#"
            SELECT
                id,
                game,
                player_one,
                player_two,
                score_one,
                score_two,
                challenge,
                deleted,
                millis AS "millis: types::Millis",
                created_ms AS "created_ms: types::Millis"
            FROM
                history
            WHERE
                game = $1
            ORDER BY
                created_ms DESC
            "#,
            game,
        )
        .fetch_all(&self.store.pool)
        .await
        .map_err(Error::from)
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
        let mut tx = self.store.pool.begin().await?;
        let games = Self::execute_refresh(None, default_rating, rating_updater, &mut tx).await?;
        tx.commit().await?;

        if !games.is_empty() {
            self.store.update_version();
        }

        Ok(games)
    }
}

impl Games<'_> {
    async fn execute_refresh<F>(
        from: Option<types::Millis>,
        default_rating: f64,
        rating_updater: F,
        tx: &mut sqlx::Transaction<'static, sqlx::Sqlite>,
    ) -> Result<Vec<types::Game>>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> f64,
    {
        let updates = Self::build_updates(from, default_rating, rating_updater, tx).await?;

        if let Some(mut query) = build_update_query(&updates) {
            query
                .build_query_as()
                .persistent(false)
                .fetch_all(tx.as_mut())
                .await
                .map_err(Into::into)
        } else {
            Ok(Vec::new())
        }
    }

    async fn build_updates<F>(
        from: Option<types::Millis>,
        default_rating: f64,
        rating_updater: F,
        tx: &mut sqlx::Transaction<'static, sqlx::Sqlite>,
    ) -> Result<Vec<RatingUpdate>>
    where
        F: Copy + Fn(f64, f64, bool, bool) -> f64,
    {
        macro_rules! f64_ne {
            ($one: expr, $two: expr) => {
                ($one - $two).abs() > f64::EPSILON
            };
        }

        let (updates, mut last_ratings) = Self::prepare_updates(from, tx).await?;

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

    async fn prepare_updates(
        from: Option<types::Millis>,
        tx: &mut sqlx::Transaction<'static, sqlx::Sqlite>,
    ) -> Result<(Vec<types::Game>, std::collections::HashMap<types::Id, f64>)> {
        if let Some(from) = from {
            let updates = sqlx::query_as!(
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
                WHERE
                    millis >= $1
                ORDER BY
                    millis ASC
                "#,
                from,
            )
            .fetch_all(tx.as_mut())
            .await?;

            let last_ratings = sqlx::query!(
                r#"
                WITH
                    ratings AS (
                        SELECT
                            player_one,
                            player_two,
                            rating_one + rating_delta AS rating_one,
                            rating_two - rating_delta AS rating_two,
                            MAX(millis) AS millis
                        FROM
                            games
                        WHERE
                            millis < $1
                            AND NOT deleted
                        GROUP BY
                            player_one,
                            player_two
                    ),
                    unified AS (
                        SELECT
                            player_one AS player,
                            rating_one AS rating,
                            millis
                        FROM
                            ratings
                        UNION
                            SELECT
                                player_two AS player,
                                rating_two AS rating,
                                millis
                            FROM
                                ratings
                    )
                SELECT
                    player AS "player!: types::Id",
                    rating AS "rating!: f64",
                    MAX(millis) AS "millis!: types::Millis"
                FROM
                    unified
                GROUP BY
                    player
                "#,
                from,
            )
            .map(|r| (r.player, r.rating))
            .fetch_all(tx.as_mut())
            .await?
            .into_iter()
            .collect();

            Ok((updates, last_ratings))
        } else {
            Self::list_games(tx.as_mut())
                .await
                .map(|games| (games, std::collections::HashMap::default()))
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
                millis,
                ignore,
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
