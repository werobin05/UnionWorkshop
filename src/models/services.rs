use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::models::service_code::ServiceCode;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Services {
    pub service_id: i64,
    pub name_service: String,
    pub code: ServiceCode,
    pub price: f64,
    pub created_at: Option<NaiveDateTime>,
}