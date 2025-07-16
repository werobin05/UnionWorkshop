use sqlx::FromRow;
use rust_decimal::Decimal;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::models::service_code::ServiceCode;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Services {
    pub service_id: i32,
    pub name_service: String,
    pub code: ServiceCode,
    pub price: Decimal,
    pub created_at: Option<NaiveDateTime>,
}