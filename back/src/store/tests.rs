use super::model;

async fn insert(
    email: &str,
    pool: &sqlx::sqlite::SqlitePool,
) -> Result<model::Player, sqlx::Error> {
    sqlx::query_as!(
        model::Player,
        r#"
        INSERT INTO players (
            name,
            email
        ) VALUES (
            "name",
            $1
        ) RETURNING
            id,
            name,
            email,
            created_ms AS "created_ms: model::Millis"
        "#,
        email
    )
    .fetch_one(pool)
    .await
}

mod constraints {
    use super::{insert, model};

    #[sqlx::test]
    async fn text_column_cannot_be_blank(pool: sqlx::sqlite::SqlitePool) {
        match insert("", &pool).await.err().unwrap() {
            sqlx::Error::Database(db) => {
                assert_eq!("275", db.code().unwrap());
                assert_eq!(
                    "CHECK constraint failed: LENGTH(TRIM(email)) > 0",
                    db.message()
                );
            }
            err => panic!("Unexpected error: {err:?}"),
        }

        match insert("    ", &pool).await.err().unwrap() {
            sqlx::Error::Database(db) => {
                assert_eq!("275", db.code().unwrap());
                assert_eq!(
                    "CHECK constraint failed: LENGTH(TRIM(email)) > 0",
                    db.message()
                );
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn not_null_cannot_be_null(pool: sqlx::sqlite::SqlitePool) {
        let error = sqlx::query_as!(
            model::Player,
            r#"
            INSERT INTO players (
                name,
                email
            ) VALUES (
                "bla",
                NULL
            ) RETURNING
                id,
                name,
                email,
                created_ms AS "created_ms: model::Millis"
            "#
        )
        .fetch_one(&pool)
        .await
        .unwrap_err();

        match error {
            sqlx::Error::Database(db) => {
                assert_eq!("1299", db.code().unwrap());
                assert_eq!("NOT NULL constraint failed: players.email", db.message());
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn unique_column_must_be_unique(pool: sqlx::sqlite::SqlitePool) {
        assert!(insert("email", &pool).await.err().is_none());
        match insert("email", &pool).await.err().unwrap() {
            sqlx::Error::Database(db) => {
                assert_eq!("2067", db.code().unwrap());
                assert_eq!("UNIQUE constraint failed: players.email", db.message());
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn foreign_key_must_exist(pool: sqlx::sqlite::SqlitePool) {
        match sqlx::query_as!(
            model::Rating,
            r#"
            INSERT INTO ratings (
                player,
                rating,
                deviation,
                volatility
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            ) RETURNING
                player,
                rating,
                deviation,
                volatility
            "#,
            0,
            0,
            0,
            0
        )
        .fetch_one(&pool)
        .await
        .unwrap_err()
        {
            sqlx::Error::Database(db) => {
                assert_eq!("787", db.code().unwrap());
                assert_eq!("FOREIGN KEY constraint failed", db.message());
            }
            err => panic!("Unexpected error: {err:?}"),
        }
    }

    #[sqlx::test]
    async fn cascade_deletes(pool: sqlx::sqlite::SqlitePool) {
        let player = insert("email", &pool).await.unwrap();

        let ranking = sqlx::query_as!(
            model::Rating,
            r#"
            INSERT INTO ratings (
                player,
                rating,
                deviation,
                volatility
            ) VALUES (
                $1,
                $2,
                $3,
                $4
            ) RETURNING
                player,
                rating,
                deviation,
                volatility
            "#,
            player.id,
            0,
            0,
            0
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(
            ranking,
            model::Rating {
                player: player.id,
                rating: 0.0,
                deviation: 0.0,
                volatility: 0.0,
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
            model::Rating,
            r#"
            SELECT
                player,
                rating,
                deviation,
                volatility
            FROM
                ratings
            "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap()
        .is_empty());
    }
}

mod behavior {
    use super::{insert, model};

    #[sqlx::test]
    async fn updates_dont_return_optional(pool: sqlx::sqlite::SqlitePool) {
        let player = insert("email", &pool).await.unwrap();

        let id = sqlx::query_as!(
            model::Id,
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
            model::Id,
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
    }
}
