use sqlx::query_as;
use sqlx::PgPool;
use crate::models::{Orders, ServiceType};

pub async fn create_order(
    pool: &PgPool,
    user_id: i64,
    service_type: ServiceType,
    package_name: String,
    price: i32,
) -> Result<Orders, sqlx::Error> {
    let order: Orders = query_as::<_, Orders>(
        r#"
        INSERT INTO orders (user_id, service_type, package_name, price)
        VALUES ($1, $2::service_type, $3, $4)
        RETURNING order_id, user_id,
                service_type as "service_type: ServiceType",
                package_name, price, created_at
        "#,
    )
    .bind(user_id)
    .bind(service_type)
    .bind(package_name)
    .bind(price)
    .fetch_one(pool)
    .await?;


    Ok(order)
}
