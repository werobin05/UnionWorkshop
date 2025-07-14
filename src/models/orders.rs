use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::models::service_type::ServiceType;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Orders {
    pub order_id: i64,
    pub user_id: i64,
    pub service_type: ServiceType,
    pub package_name: String,
    pub price: i32,
    pub created_at: Option<NaiveDateTime>,
}