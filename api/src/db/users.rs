use super::models::User;
use sqlx::{query, query_as};

pub async fn insert_user(nickname: String, email: String, secret: String) -> anyhow::Result<i64> {
    let mut conn = super::common::get_sqlite_connection().await?;

    let id: _ = query("INSERT INTO users ( nickname, email, secret ) VALUES ( ?, ?, ? )")
        .bind(nickname)
        .bind(email)
        .bind(secret)
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

    Ok(id)
}

pub async fn get_user_by_id(id: i64) -> anyhow::Result<User> {
    let mut conn = super::common::get_sqlite_connection().await?;

    let user = query_as::<_, User>(
        "
        SELECT *
        FROM users 
        WHERE id=?
        ",
    )
    .bind(id)
    .fetch_one(&mut conn)
    .await
    .expect(format!("get user by id={}", id).as_str());

    Ok(user)
}

// Get all users
pub async fn get_users() -> anyhow::Result<Vec<User>> {
    let mut conn = super::common::get_sqlite_connection().await?;

    let users = query_as::<_, User>(
        "
        SELECT *
        FROM users
        ",
    )
    .fetch_all(&mut conn)
    .await
    .expect("get users");

    Ok(users)
}
