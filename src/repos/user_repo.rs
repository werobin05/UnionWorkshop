use sqlx::{PgPool, Result};
use crate::models::Users;

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<Option<Users>> {
    let user = sqlx::query_as::<_, Users>(
        r#"
        SELECT * FROM "Users"
        WHERE username = $1
        "#
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(pool: &PgPool, username: &str, email: &str) -> Result<()> {
    sqlx::query(
        r#"
            INSERT INTO "Users" (username, email, created_at)
            VALUES ($1, $2, NOW())
            "#
    )
    .bind(username)
    .bind(email)
    .execute(pool)
    .await?;

    Ok(())
}