use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn orders(bot: Bot, msg: Message) -> ResponseResult<()> {
    let inline_keyboard = InlineKeyboardMarkup::new( vec![
        vec! [
            InlineKeyboardButton::callback("✅ Выполенные", "complected"),
            InlineKeyboardButton::callback("🔄 В разработке", "in_process"),
        ],
        vec! [
            InlineKeyboardButton::callback("❌ Отмененные", "cancelled"),
        ]
    ]);
    bot.send_message(msg.chat.id, "🗂️ *История заказов*")
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(inline_keyboard)
        .await?;
    Ok(())
}