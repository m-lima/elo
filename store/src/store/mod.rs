mod users;

use super::{Error, Result};

#[derive(Debug, Clone)]
pub struct Store {
    pool: sqlx::sqlite::SqlitePool,
}

impl Store {
    pub async fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(path)
            .optimize_on_close(true, Some(1000))
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .map_err(Error::Connection)?;

        Ok(Self { pool })
    }

    #[must_use]
    pub fn users(&self) -> users::Users<'_> {
        users::Users::from(self)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod constraints {

        async fn insert(
            email: &str,
            pool: &sqlx::sqlite::SqlitePool,
        ) -> Result<crate::model::User, sqlx::Error> {
            sqlx::query_as!(
                crate::model::User,
                r#"
                INSERT INTO users (
                    name,
                    email
                ) VALUES (
                    "name",
                    $1
                ) RETURNING
                    id,
                    name,
                    email,
                    created_ms AS "created_ms: crate::model::Millis"
                "#,
                email
            )
            .fetch_one(pool)
            .await
        }

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
                crate::model::User,
                r#"
                    INSERT INTO users (
                        name,
                        email
                    ) VALUES (
                        "bla",
                        NULL
                    ) RETURNING
                        id,
                        name,
                        email,
                        created_ms AS "created_ms: crate::model::Millis"
                    "#
            )
            .fetch_one(&pool)
            .await
            .unwrap_err();

            match error {
                sqlx::Error::Database(db) => {
                    assert_eq!("1299", db.code().unwrap());
                    assert_eq!("NOT NULL constraint failed: users.email", db.message());
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
                    assert_eq!("UNIQUE constraint failed: users.email", db.message());
                }
                err => panic!("Unexpected error: {err:?}"),
            }
        }

        #[sqlx::test]
        async fn foreign_key_must_exist(pool: sqlx::sqlite::SqlitePool) {
            match sqlx::query_as!(
                crate::model::Ranking,
                r#"
                INSERT INTO rankings (
                    user,
                    score,
                    wins,
                    losses,
                    points_won,
                    points_lost
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
                ) RETURNING
                    user,
                    score,
                    wins,
                    losses,
                    points_won,
                    points_lost
                "#,
                0,
                0,
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
            let user = insert("email", &pool).await.unwrap();

            let ranking = sqlx::query_as!(
                crate::model::Ranking,
                r#"
                INSERT INTO rankings (
                    user,
                    score,
                    wins,
                    losses,
                    points_won,
                    points_lost
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
                ) RETURNING
                    user,
                    score,
                    wins,
                    losses,
                    points_won,
                    points_lost
                "#,
                user.id,
                0,
                0,
                0,
                0,
                0
            )
            .fetch_one(&pool)
            .await
            .unwrap();

            assert_eq!(
                ranking,
                crate::model::Ranking {
                    user: user.id,
                    score: 0,
                    wins: 0,
                    losses: 0,
                    points_won: 0,
                    points_lost: 0,
                }
            );

            assert_eq!(
                Some(user.id),
                sqlx::query!(
                    r#"
                        DELETE FROM
                            users
                        WHERE
                            id = $1
                        RETURNING
                            id
                    "#,
                    user.id
                )
                .map(|r| r.id)
                .fetch_optional(&pool)
                .await
                .unwrap()
            );

            assert!(sqlx::query_as!(
                crate::model::Ranking,
                r#"
                SELECT
                    user,
                    score,
                    wins,
                    losses,
                    points_won,
                    points_lost
                FROM
                    rankings
                "#,
            )
            .fetch_all(&pool)
            .await
            .unwrap()
            .is_empty());
        }
    }
}
