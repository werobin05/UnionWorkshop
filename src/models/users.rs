use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct  Users {
    pub(crate) user_id: i32,
    username: String,
    email: String,
    created_at: Option<NaiveDateTime>,
}