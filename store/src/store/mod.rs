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

        async fn insert(email: &str, pool: &sqlx::sqlite::SqlitePool) -> Option<sqlx::Error> {
            sqlx::query_as!(
                crate::model::User,
                r#"
                INSERT INTO users (
                    email
                ) VALUES (
                    $1
                ) RETURNING
                    id,
                    email,
                    created_ms AS "created_ms: crate::model::Millis"
                "#,
                email
            )
            .fetch_one(pool)
            .await
            .err()
        }

        #[sqlx::test]
        async fn text_column_cannot_be_blank(pool: sqlx::sqlite::SqlitePool) {
            match insert("", &pool).await.unwrap() {
                sqlx::Error::Database(db) => {
                    assert_eq!("275", db.code().unwrap());
                    assert_eq!(
                        "CHECK constraint failed: LENGTH(TRIM(email)) > 0",
                        db.message()
                    );
                }
                err => panic!("Unexpected error: {err:?}"),
            }

            match insert("    ", &pool).await.unwrap() {
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
                        email
                    ) VALUES (
                        NULL
                    ) RETURNING
                        id,
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
            assert!(insert("email", &pool).await.is_none());
            match insert("email", &pool).await.unwrap() {
                sqlx::Error::Database(db) => {
                    assert_eq!("2067", db.code().unwrap());
                    assert_eq!("UNIQUE constraint failed: users.email", db.message());
                }
                err => panic!("Unexpected error: {err:?}"),
            }
        }
    }
}
