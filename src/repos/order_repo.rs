use sqlx::PgPool;
use crate::models::{UserOrderInfo};

pub async fn fetch_orders(
    pool: &PgPool,
    user_id: i32,
    page: usize,
    per_page: usize,
) -> sqlx::Result<Vec<UserOrderInfo>> {
    let offset = (page.saturating_sub(1) * per_page) as i64;
    let limit = per_page as i64;

    let orders = sqlx::query_as::<_, UserOrderInfo>(
        r#"
        SELECT 
            o.order_id,
            s.name_service,
            s.code,
            o.status,
            s.price,
            o.created_at
        FROM "Orderes" o
        LEFT JOIN "Services" s ON o.service_id = s.service_id
        WHERE o.user_id = $1
        ORDER BY o.created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(orders)
}
