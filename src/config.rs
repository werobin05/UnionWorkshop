use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Environment loaded and logger initialized");
}

pub fn get_token() -> String {
    env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN must be set in .env")
}