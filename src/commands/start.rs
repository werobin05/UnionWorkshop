use teloxide::prelude::*;
use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub async fn start(bot: Bot, msg: Message) -> ResponseResult<()> {
    let keyboard = KeyboardMarkup {
        keyboard: vec![
            vec![
                KeyboardButton::new("📦 Услуги"),
                KeyboardButton::new("❓ F.A.Q"),
            ],
            vec![KeyboardButton::new("📩 Написать в поддержку")],
        ],
        resize_keyboard: true,
        one_time_keyboard: false,
        selective: false,
        is_persistent: true,
        input_field_placeholder: "Выберите действие 📍".to_string(),
    };

    bot.send_message(msg.chat.id, "Добро пожаловать! 👋\n\n\
        Чтобы сделать заказ, нажмите /services.\n\
        Доступные команды — /help.\n\n\
        Если возникли вопросы, напишите нам на почту.")
        .reply_markup(keyboard)
        .await?;

    Ok(())
}
