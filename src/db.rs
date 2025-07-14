use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db_pool() -> PgPool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}