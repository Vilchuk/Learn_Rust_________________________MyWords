use crate::app_config::AppConfig;
use sqlx::{pool::PoolConnection, Sqlite, SqlitePool};

pub async fn get_sqlite_connection() -> anyhow::Result<PoolConnection<Sqlite>> {
    let config = AppConfig::get().expect("something went wrong");
    let pool = SqlitePool::connect(config.db_connection_string.as_str()).await?;
    let conn = pool.acquire().await?;
    Ok(conn)
}
