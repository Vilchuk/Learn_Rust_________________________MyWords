use super::models::User;
use crate::app_config::AppConfig;
use sqlx::{
    query, query_as,
    sqlite::{SqlitePool, SqliteRow},
    Row,
};
use std::env;

pub struct Client {
    pool: SqlitePool,
}

impl Client {
    pub async fn new() -> anyhow::Result<Client> {
        let config = AppConfig::get().expect("something went wrong");
        let pool = SqlitePool::connect(config.db_connection_string.as_str()).await?;

        Ok(Client { pool })
    }

    pub async fn insert_user(
        &self,
        name: String,
        email: String,
        secret: String,
    ) -> anyhow::Result<i64> {
        let mut conn = self.pool.acquire().await?;

        let id: _ = query("INSERT INTO users ( name, email, secret ) VALUES ( ?, ?, ? )")
            .bind(name)
            .bind(email)
            .bind(secret)
            .execute(&mut conn)
            .await?
            .last_insert_rowid();

        Ok(id)
    }

    pub async fn get_user_by_id(&self, id: i64) -> anyhow::Result<User> {
        let mut conn = self.pool.acquire().await?;
        let user = query(
            "
            SELECT id,name,email,secret 
            FROM users 
            WHERE id=?
            ",
        )
        .bind(id)
        .map(|row: SqliteRow| User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            secret: row.get("secret"),
        })
        .fetch_one(&mut conn)
        .await?;

        Ok(user)
    }
}
