use sqlx::Type;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, Display};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Display, EnumString, Type)]
#[sqlx(type_name = "service_code")]
#[sqlx(rename_all = "lowercase")]
#[strum(serialize_all = "snake_case")]
pub enum StatusOrder {
    InProgress,
    Completed,
    Cancelled,
}
