use crate::types;

async fn insert(
    name: &str,
    email: &str,
    pool: &sqlx::sqlite::SqlitePool,
) -> Result<types::Player, sqlx::Error> {
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
            0
        ) RETURNING
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
        "#,
        name,
        email
    )
    .fetch_one(pool)
    .await
}

mod constraints {
    use super::{insert, types};

    #[sqlx::test]
    async fn text_column_cannot_be_blank(pool: sqlx::sqlite::SqlitePool) {
        match insert("name", "", &pool).await.err().unwrap() {
            sqlx::Error::Database(db) => {
                assert_eq!(
                    "CHECK constraint failed: LENGTH(TRIM(email)) > 0 AND LENGTH(email) <= 128",
                    db.message()
                );
                assert_eq!("275", db.code().unwrap());
            }
            err => panic!("Unexpected error: {err:?}"),
        }

        match insert("name", "    ", &pool).await.err().unwrap() {
            sqlx::Error::Database(db) => {
                assert_eq!(
                    "CHECK constraint failed: LENGTH(TRIM(email)) > 0 AND LENGTH(email) <= 128",
                    db.message()
                );
                assert_eq!("275", db.code().unwrap());
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn not_null_cannot_be_null(pool: sqlx::sqlite::SqlitePool) {
        let error = sqlx::query_as!(
            types::Player,
            r#"
            INSERT INTO players (
                name,
                email,
                rating
            ) VALUES (
                "bla",
                NULL,
                0
            ) RETURNING
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
            "#
        )
        .fetch_one(&pool)
        .await
        .unwrap_err();

        match error {
            sqlx::Error::Database(db) => {
                assert_eq!("NOT NULL constraint failed: players.email", db.message());
                assert_eq!("1299", db.code().unwrap());
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn unique_column_must_be_unique(pool: sqlx::sqlite::SqlitePool) {
        assert!(insert("name", "email", &pool).await.err().is_none());
        match insert("name", "email", &pool).await.err().unwrap() {
            sqlx::Error::Database(db) => {
                assert_eq!("UNIQUE constraint failed: players.email", db.message());
                assert_eq!("2067", db.code().unwrap());
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn foreign_key_must_exist(pool: sqlx::sqlite::SqlitePool) {
        match sqlx::query_as!(
            types::Invite,
            r#"
            INSERT INTO invites (
                inviter,
                name,
                email
            ) VALUES (
                0,
                "bla",
                "email"
            ) RETURNING
                id,
                inviter,
                name,
                email,
                created_ms AS "created_ms: types::Millis"
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap_err()
        {
            sqlx::Error::Database(db) => {
                assert_eq!("FOREIGN KEY constraint failed", db.message());
                assert_eq!("787", db.code().unwrap());
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn exclusive_values_must_be_different(pool: sqlx::sqlite::SqlitePool) {
        let player = insert("name", "email", &pool).await.unwrap();

        match sqlx::query_as!(
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
                $1,
                0,
                0,
                0,
                0
            ) RETURNING
                id,
                player_one,
                player_two,
                score_one,
                score_two,
                rating_one,
                rating_two,
                created_ms AS "created_ms: types::Millis"
            "#,
            player.id
        )
        .fetch_one(&pool)
        .await
        .unwrap_err()
        {
            sqlx::Error::Database(db) => {
                assert_eq!(
                    "CHECK constraint failed: player_one <> player_two",
                    db.message()
                );
                assert_eq!("275", db.code().unwrap());
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn scores_must_be_valid(pool: sqlx::sqlite::SqlitePool) {
        let player_one = insert("one", "one", &pool).await.unwrap();
        let player_two = insert("two", "two", &pool).await.unwrap();

        for one in 0..13 {
            for two in 0..13 {
                let result = sqlx::query_as!(
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
                        0,
                        0
                    ) RETURNING
                        id,
                        player_one,
                        player_two,
                        score_one,
                        score_two,
                        rating_one,
                        rating_two,
                        created_ms AS "created_ms: types::Millis"
                    "#,
                    player_one.id,
                    player_two.id,
                    one,
                    two,
                )
                .fetch_one(&pool)
                .await;

                println!("Scores: [{one:02} x {two:02}]");
                if (one == 12 && two == 10)
                    || (one == 10 && two == 12)
                    || (one == 11 && two < 11)
                    || (one < 11 && two == 11)
                {
                    result.unwrap();
                } else {
                    match result.unwrap_err() {
                        sqlx::Error::Database(db) => {
                            assert_eq!(
                                "CHECK constraint failed: (score_one = 11 AND score_two < 11)\n      OR (score_one = 12 AND score_two = 10)\n      OR (score_one < 11 AND score_two = 11)\n      OR (score_one = 10 AND score_two = 12)",
                                db.message()
                            );
                            assert_eq!("275", db.code().unwrap());
                        }
                        err => panic!("Unexpected error: {err:?}"),
                    }
                }
            }
        }
    }

    #[sqlx::test]
    async fn cascade_deletes(pool: sqlx::sqlite::SqlitePool) {
        let player = insert("name", "email", &pool).await.unwrap();

        let invite = sqlx::query_as!(
            types::Invite,
            r#"
            INSERT INTO invites (
                inviter,
                name,
                email
            ) VALUES (
                $1,
                "namer",
                "emailer"
            ) RETURNING
                id,
                inviter,
                name,
                email,
                created_ms AS "created_ms: types::Millis"
            "#,
            player.id,
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(
            invite,
            types::Invite {
                id: invite.id,
                inviter: player.id,
                name: String::from("namer"),
                email: String::from("emailer"),
                created_ms: invite.created_ms,
            }
        );

        assert_eq!(
            Some(player.id),
            sqlx::query!(
                r#"
                DELETE FROM
                    players
                WHERE
                    id = $1
                RETURNING
                    id
                "#,
                player.id
            )
            .map(|r| r.id)
            .fetch_optional(&pool)
            .await
            .unwrap()
        );

        assert!(sqlx::query_as!(
            types::Invite,
            r#"
            SELECT
                id,
                inviter,
                name,
                email,
                created_ms AS "created_ms: types::Millis"
            FROM
                invites
            "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap()
        .is_empty());
    }

    #[sqlx::test]
    async fn cascade_set_null(pool: sqlx::sqlite::SqlitePool) {
        let player = insert("name", "email", &pool).await.unwrap();

        let new_player = sqlx::query_as!(
            types::Player,
            r#"
            INSERT INTO players (
                inviter,
                name,
                email,
                rating
            ) VALUES (
                $1,
                "namer",
                "emailer",
                0
            ) RETURNING
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
            "#,
            player.id,
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(
            new_player,
            types::Player {
                id: new_player.id,
                name: String::from("namer"),
                email: String::from("emailer"),
                inviter: Some(player.id),
                rating: 0.0,
                wins: 0,
                losses: 0,
                points_won: 0,
                points_lost: 0,
                created_ms: new_player.created_ms,
            }
        );

        assert_eq!(
            Some(player.id),
            sqlx::query!(
                r#"
                DELETE FROM
                    players
                WHERE
                    id = $1
                RETURNING
                    id
                "#,
                player.id
            )
            .map(|r| r.id)
            .fetch_optional(&pool)
            .await
            .unwrap()
        );

        let new_player = sqlx::query_as!(
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
            "#,
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(
            new_player,
            types::Player {
                id: new_player.id,
                name: String::from("namer"),
                email: String::from("emailer"),
                inviter: None,
                rating: 0.0,
                wins: 0,
                losses: 0,
                points_won: 0,
                points_lost: 0,
                created_ms: new_player.created_ms,
            }
        );
    }
}

mod behavior {
    use super::insert;

    #[sqlx::test]
    async fn updates_dont_return_optional(pool: sqlx::sqlite::SqlitePool) {
        let player = insert("name", "email", &pool).await.unwrap();

        let id = sqlx::query_as!(
            super::super::Id,
            r#"
            UPDATE
                players
            SET
                name = "other"
            WHERE
                id = $1
            RETURNING
                id AS "id!: _"
            "#,
            player.id
        )
        .fetch_optional(&pool)
        .await
        .map(|r| r.map(|id| id.id))
        .unwrap();

        assert_eq!(id, Some(player.id));

        let id = sqlx::query_as!(
            super::super::Id,
            r#"
            UPDATE
                players
            SET
                name = "other"
            WHERE
                id = 27
            RETURNING
                id AS "id!: _"
            "#
        )
        .fetch_optional(&pool)
        .await
        .map(|r| r.map(|id| id.id))
        .unwrap();

        assert_eq!(id, None);

        let error = sqlx::query_as!(
            super::super::Id,
            r#"
            UPDATE
                players
            SET
                name = "other"
            WHERE
                id = 27
            RETURNING
                id AS "id!: _"
            "#
        )
        .fetch_one(&pool)
        .await
        .map(|r| r.id)
        .unwrap_err();

        assert!(matches!(error, sqlx::Error::RowNotFound));
    }
}
