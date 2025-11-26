use super::super::error::Error;
use crate::{rating, types};

type Result<T = ()> = std::result::Result<T, Error>;

pub struct Games<'a, R>
where
    R: rating::Config,
{
    store: &'a super::Store<R>,
}

impl<'a, R> From<&'a super::Store<R>> for Games<'a, R>
where
    R: rating::Config,
{
    fn from(store: &'a super::Store<R>) -> Self {
        Self { store }
    }
}

impl<R> Games<'_, R>
where
    R: rating::Config,
{
    // TODO: This would allow the front end to not have to fetch all games
    // TODO: This would also mean moving the EnrichedPlayer to the backend, so not all games need
    // to be loaded
    #[tracing::instrument(skip(self))]
    pub async fn list(&self) -> Result<Vec<types::Game>> {
        Self::list_games(&self.store.pool).await
    }

    #[tracing::instrument(skip(self,))]
    pub async fn register(
        &self,
        (player_one, player_two): (types::Id, types::Id),
        (score_one, score_two): (u8, u8),
        challenge: bool,
        millis: types::Millis,
    ) -> Result<(types::Game, Vec<types::Game>, Vec<types::Rating>)> {
        validate_game(player_one, player_two, score_one, score_two)?;

        let mut tx = self.store.pool.begin().await?;

        if challenge {
            Self::validate_challenge(player_one, player_two, millis, None, tx.as_mut()).await?;
        }

        let game = sqlx::query_as!(
            types::Game,
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
            "#,
            player_one,
            player_two,
            score_one,
            score_two,
            challenge,
            millis,
        )
        .fetch_one(tx.as_mut())
        .await?;

        let mut updates = Self::execute_refresh(Some(millis), &mut tx).await?;

        let game = match updates.iter().position(|g| g.id == game.id) {
            Some(idx) => updates.swap_remove(idx),
            None => game,
        };

        let ratings = Self::ratings_at(types::Millis::now(), tx.as_mut(), true).await?;

        tx.commit().await?;

        self.store.update_version();

        Ok((game, updates, ratings))
    }

    #[tracing::instrument(skip(self))]
    pub async fn update(
        &self,
        game: types::Game,
    ) -> Result<(types::Game, Vec<types::Game>, Vec<types::Rating>)> {
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

        let mut updates = Self::execute_refresh(Some(old_millis.min(new_millis)), &mut tx).await?;

        let game = match updates.iter().position(|g| g.id == game.id) {
            Some(idx) => updates.swap_remove(idx),
            None => game,
        };

        let ratings = Self::ratings_at(types::Millis::now(), tx.as_mut(), true).await?;

        tx.commit().await?;

        self.store.update_version();

        Ok((game, updates, ratings))
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

    #[tracing::instrument(skip(self))]
    pub async fn refresh(&self) -> Result<Vec<types::Game>> {
        let mut tx = self.store.pool.begin().await?;
        let games = Self::execute_refresh(None, &mut tx).await?;
        tx.commit().await?;

        if !games.is_empty() {
            self.store.update_version();
        }

        Ok(games)
    }

    pub async fn ratings_at<'c, 'e, E>(
        at: types::Millis,
        executor: E,
        decay: bool,
    ) -> Result<Vec<types::Rating>>
    where
        'c: 'e,
        E: 'e + sqlx::Executor<'c, Database = sqlx::Sqlite>,
    {
        let millis = if decay {
            types::Millis::from(i64::MAX)
        } else {
            at
        };

        sqlx::query_as!(
            types::Rating,
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
                        NOT deleted
                        AND millis < $1
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
                MAX(millis) AS "last_game!: types::Millis"
            FROM
                unified
            GROUP BY
                player
            ORDER BY
                player
            "#,
            millis,
        )
        .map(|r| {
            if decay {
                types::Rating {
                    rating: R::decayer(r.last_game, at, r.rating),
                    ..r
                }
            } else {
                r
            }
        })
        .fetch_all(executor)
        .await
        .map_err(Error::from)
    }
}

impl<R> Games<'_, R>
where
    R: rating::Config,
{
    async fn execute_refresh(
        from: Option<types::Millis>,
        tx: &mut sqlx::Transaction<'static, sqlx::Sqlite>,
    ) -> Result<Vec<types::Game>> {
        let updates = Self::build_updates(from, tx).await?;

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

    async fn build_updates(
        from: Option<types::Millis>,
        tx: &mut sqlx::Transaction<'static, sqlx::Sqlite>,
    ) -> Result<Vec<RatingUpdate>> {
        use crate::macros::f64;

        let (updates, mut last_ratings) = Self::prepare_updates(from, tx).await?;

        Ok(updates
            .into_iter()
            .filter_map(|game| {
                let decay = |r: &types::Rating| R::decayer(r.last_game, game.millis, r.rating);

                let rating_one_idx =
                    last_ratings.binary_search_by_key(&game.player_one, |r| r.player);
                let rating_two_idx =
                    last_ratings.binary_search_by_key(&game.player_two, |r| r.player);

                let rating_one = rating_one_idx
                    .ok()
                    .and_then(|i| last_ratings.get(i))
                    .map_or(R::DEFAULT_VALUE, decay);
                let rating_two = rating_two_idx
                    .ok()
                    .and_then(|i| last_ratings.get(i))
                    .map_or(R::DEFAULT_VALUE, decay);

                let rating_delta = if game.deleted {
                    0.0
                } else {
                    R::updater(
                        rating_one,
                        rating_two,
                        game.score_one > game.score_two,
                        game.challenge,
                    )
                };

                let rating_two_idx = match rating_one_idx {
                    Ok(i) => {
                        let r = &mut last_ratings[i];
                        r.rating = rating_one + rating_delta;
                        r.last_game = game.millis;
                        rating_two_idx
                    }
                    Err(i) => {
                        last_ratings.insert(
                            i,
                            types::Rating {
                                player: game.player_one,
                                rating: rating_one + rating_delta,
                                last_game: game.millis,
                            },
                        );
                        if game.player_one < game.player_two {
                            rating_two_idx.map(|i| i + 1).map_err(|i| i + 1)
                        } else {
                            rating_two_idx
                        }
                    }
                };

                match rating_two_idx {
                    Ok(i) => {
                        let r = &mut last_ratings[i];
                        r.rating = rating_two - rating_delta;
                        r.last_game = game.millis;
                    }
                    Err(i) => {
                        last_ratings.insert(
                            i,
                            types::Rating {
                                player: game.player_two,
                                rating: rating_two - rating_delta,
                                last_game: game.millis,
                            },
                        );
                    }
                }

                (f64!(ne rating_one, game.rating_one)
                    || f64!(ne rating_two, game.rating_two)
                    || f64!(ne rating_delta, game.rating_delta))
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
    ) -> Result<(Vec<types::Game>, Vec<types::Rating>)> {
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

            let last_ratings = Self::ratings_at(from, tx.as_mut(), false).await?;

            Ok((updates, last_ratings))
        } else {
            Self::list_games(tx.as_mut())
                .await
                .map(|games| (games, Vec::new()))
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
        let challenged_this_week = if let Some(ignore) = ignore {
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
                    AND STRFTIME('%Y%W', $3 / 1000, 'unixepoch') = STRFTIME('%Y%W', millis / 1000, 'unixepoch')
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
                    AND STRFTIME('%Y%W', $3 / 1000, 'unixepoch') = STRFTIME('%Y%W', millis / 1000, 'unixepoch')
                "#,
                player_one,
                player_two,
                millis,
            )
            .fetch_optional(executor)
            .await?
            .is_some()
        };

        if challenged_this_week {
            return Err(Error::InvalidValue(
                "Players cannot challenge each other more than once a week",
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
