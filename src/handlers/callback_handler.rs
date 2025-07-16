use sqlx::PgPool;
use teloxide::prelude::*;
use crate::models::ServiceCode;
use crate::ui::keyboard::keyboard_from_services;
use crate::repos::service_repo::fetch_by_category;
use teloxide::types::{CallbackQuery, InlineKeyboardMarkup};

pub async fn callback_handler(bot: Bot, q: CallbackQuery, pool: PgPool) -> ResponseResult<()> {
    let data = q.data.clone().unwrap_or_else(|| "<empty>".into());

    bot.answer_callback_query(q.id.clone())
        .text(format!("Вы выбрали: {}", data))
        .await?;

    if let Some(msg) = q.message {
        let chat = msg.chat();

        match data.as_str() {
            "service_design" | "service_dev" | "service_marketing" => {
                let category_code = match data.as_str() {
                    "service_design" => ServiceCode::Design,
                    "service_dev" => ServiceCode::Dev,
                    "service_marketing" => ServiceCode::Marketing,
                    _ => {
                        bot.send_message(chat.id, "Неизвестная категория сервиса.")
                            .await?;
                        return Ok(());
                    }
                };
                match fetch_by_category(&pool, category_code).await {
                    Ok(services) => {
                        let keyboard: InlineKeyboardMarkup = keyboard_from_services(&services);
                        bot.send_message(chat.id, "Выберите пакет:")
                            .reply_markup(keyboard)
                            .await?;
                    }
                    Err(e) => {
                        eprintln!("❌ Ошибка при получении услуг: {:?}", e);
                        bot.send_message(chat.id, "Произошла ошибка при загрузке услуг.")
                            .await?;
                    }
                }
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
