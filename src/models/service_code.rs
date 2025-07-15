use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{EnumString, Display};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Display, EnumString, Type)]
#[sqlx(type_name = "service_code")]
#[strum(serialize_all = "snake_case")]
pub enum ServiceCode {
    #[sqlx(rename = "design")] Design,
    #[sqlx(rename = "dev")] Development,
    #[sqlx(rename = "marketing")] Marketing,
}