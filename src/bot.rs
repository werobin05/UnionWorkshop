use teloxide::{prelude::*, types::Update};
use crate::handlers::{message_handler, callback_handler};
use crate::config;

mod commands;
mod handlers;

pub async fn run() {
    let bot = Bot::new(config::get_token());
    println!("Bot running...");

    Dispatcher::builder(
        bot.clone(),
        dptree::entry()
            .branch(Update::filter_message().endpoint(message_handler))
            .branch(Update::filter_callback_query().endpoint(callback_handler))
    )
    .dependencies(dptree::deps![bot])
    .default_handler(|upd| async move {
        log::warn!("Unhandled update: {:?}", upd);
    })
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
