mod config;
mod bot;
mod handlers;
mod commands;
mod utils;
mod models;
mod services;
mod db;

#[tokio::main]
async fn main() {
    config::init();
    bot::run().await;
    db::init_db_pool().await;
}