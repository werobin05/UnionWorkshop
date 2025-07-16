use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::models::StatusOrder;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Orders {
    pub freelancer_id: i32,
    pub service_id: i32,
    pub user_id: i32,
    pub status: StatusOrder,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserOrderInfo {
    pub order_id: i32,
    pub name_service: String,
    pub code: String,
    pub status: StatusOrder,
    pub price: f64,
    pub created_at: Option<NaiveDateTime>,
}
