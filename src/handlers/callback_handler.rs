use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    let data = q.data.clone().unwrap_or_else(|| "<empty>".into());

    bot.answer_callback_query(q.id.clone())
        .text(format!("Вы выбрали: {}", data))
        .await?;

    if let Some(msg) = q.message {
        let chat = msg.chat();

        match data.as_str() {
            "service_design" => {
                let inline_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("0️⃣ mini app (от 4000₽)", "design_mini")],
                    vec![InlineKeyboardButton::callback("1️⃣ medium app (от 8000₽)", "design_medium")],
                    vec![InlineKeyboardButton::callback("2️⃣ full app (от 12000₽)", "design_full")],
                    vec![InlineKeyboardButton::callback("❇️ Custom app (от 15000₽)", "design_custom")],
                    vec![InlineKeyboardButton::callback("⬅️ Назад", "back_to_services")],
                ]);
                bot.send_message(chat.id, "Пакеты для дизайна:")
                    .reply_markup(inline_keyboard)
                    .await?;
            }
            "service_dev" => {
                let inline_keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("0️⃣ Telegram bot (5000₽)", "dev_tg")],
                    vec![InlineKeyboardButton::callback("1️⃣ BackEnd /web/ (от 10000₽)", "dev_backend")],
                    vec![InlineKeyboardButton::callback("2️⃣ DataBase /Data/ (от 5000₽)", "dev_db")],
                    vec![InlineKeyboardButton::callback("3️⃣ Mobile app /mobile/ 15000₽", "dev_mobile")],
                    vec![InlineKeyboardButton::callback("❇️ FullStack /web/ от 20000₽", "dev_full")],
                    vec![InlineKeyboardButton::callback("⬅️ Назад", "back_to_services")],
                ]);
                bot.send_message(chat.id, "Пакеты для разработки:")
                    .reply_markup(inline_keyboard)
                    .await?;
            }
            "service_marketing" => {
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![InlineKeyboardButton::callback("Базовый (4000₽)", "marketing_basic")],
                    vec![InlineKeyboardButton::callback("Премиум (9000₽)", "marketing_premium")],
                    vec![InlineKeyboardButton::callback("⬅️ Назад", "back_to_services")],
                ]);
                bot.send_message(chat.id, "Пакеты для маркетинга:")
                    .reply_markup(keyboard)
                    .await?;
            }
            "back_to_services" => {
                bot.delete_message(msg.chat().id, msg.id()).await?;
            }

            _ => {
                bot.send_message(chat.id, "Неизвестная команда, выберите из меню.")
                    .await?;
            }

        }
    }

    Ok(())
}
