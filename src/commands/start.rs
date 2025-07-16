use sqlx::PgPool;
use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup};

use crate::repos::user_repo::create_user;

pub async fn start(bot: Bot, msg: Message, pool: PgPool) -> ResponseResult<()> {
    if let Some(user) = &msg.from {
        let username = user.username.as_deref().unwrap_or("unknown");
        let email = "";

        match create_user(&pool, username, email).await {
            Ok(_) => log::info!("User created or updated successfully"),
            Err(e) => log::error!("Failed to create user: {:?}", e),
        }

        let keyboard = KeyboardMarkup {
            keyboard: vec![
                vec![
                    KeyboardButton::new("📦 Услуги"),
                    KeyboardButton::new("📋 Биржа"),
                    KeyboardButton::new("🗂️ Заказы"),
                ],
                vec![KeyboardButton::new("❓ F.A.Q")],
                vec![KeyboardButton::new("📩 Написать в поддержку")],
            ],
            resize_keyboard: true,
            one_time_keyboard: false,
            selective: false,
            is_persistent: true,
            input_field_placeholder: "Выберите действие 📍".to_string(),
        };

        bot.send_message(
            msg.chat.id,
            format!(
                "Добро пожаловать, @{}! 👋\n\n\
                 Чтобы сделать заказ, нажмите /services.\n\
                 Доступные команды — /help.\n\n\
                 Если возникли вопросы, напишите нам на почту.",
                username
            ),
        )
        .reply_markup(keyboard)
        .await?;
    }

    Ok(())
}
