use sqlx::PgPool;
use crate::config;
use crate::db::init_db_pool;
use teloxide::{prelude::*, types::Update};
use crate::handlers::{message_handler, callback_handler};

pub async fn run() {
    let bot = Bot::new(config::get_token());
    println!("Bot running...");
    let pool: PgPool = init_db_pool().await;

    Dispatcher::builder(
        bot.clone(),
        dptree::entry()
            .branch(Update::filter_message().endpoint(message_handler))
            .branch(Update::filter_callback_query().endpoint(
                move |bot: Bot, q: CallbackQuery, pool: PgPool| async move {
                    callback_handler(bot, q, pool).await
                }
            ))
    )
    .dependencies(dptree::deps![bot, pool])
    .default_handler(|upd| async move {
        log::warn!("Unhandled update: {:?}", upd);
    })
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
