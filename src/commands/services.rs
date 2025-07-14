use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn services(bot: Bot, msg: Message) -> ResponseResult<()> {
    let inline_keyboard = InlineKeyboardMarkup::new(vec! [
        vec! [
            InlineKeyboardButton::callback("🎨 Дизайн", "service_design"),
            InlineKeyboardButton::callback("👨‍💻 Разработка", "service_dev"),
        ],
        vec! [
            InlineKeyboardButton::callback("📊 Маркетинг и SMM", "service_marketing"),
        ],
        vec! [
            InlineKeyboardButton::callback("📦 Мои заказы", "my_orders")
        ]
    ]);
    bot.send_message(msg.chat.id, "🔧 *Выберите интересующую вас услугу:*")
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(inline_keyboard)
        .await?;
    Ok(())
}
