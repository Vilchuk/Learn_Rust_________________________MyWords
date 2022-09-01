use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub nickname: String,
    pub email: String,
    pub secret: String,
    pub created_at: DateTime<Utc>,
}
