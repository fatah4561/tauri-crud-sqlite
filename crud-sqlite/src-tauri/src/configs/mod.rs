use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub async fn new_sqlx_db() -> Result<SqlitePool, sqlx::Error> {
    let connect_options = SqliteConnectOptions::new()
        .filename("../db.db3")
        .create_if_missing(true);
    let conn = sqlx::sqlite::SqlitePool::connect_with(connect_options).await;
    return conn;
}
