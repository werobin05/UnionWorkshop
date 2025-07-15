use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Freelancers {
    freelancer_id: i64,
    full_name: String,
    username: String,
    description: String,
    created_at: Option<NaiveDateTime>,
}