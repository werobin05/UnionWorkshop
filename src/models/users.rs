use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct  Users {
    user_id: i64,
    username: String,
    email: String,
    created_at: Option<NaiveDateTime>,
}