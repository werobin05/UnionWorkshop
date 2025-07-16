mod config;
mod bot;
mod handlers;
mod commands;
mod utils;
mod models;
mod services;
mod db;
mod repos;
mod ui;

#[tokio::main]
async fn main() {
    config::init();
    bot::run().await;
    db::init_db_pool().await;
    tracing_subscriber::fmt::init();
}