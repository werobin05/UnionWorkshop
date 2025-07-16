use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::models::Services;

pub fn keyboard_from_services(services: &[Services]) -> InlineKeyboardMarkup {
    let mut rows: Vec<Vec<InlineKeyboardButton>> = services
        .chunks(2)
        .map(|pair| {
            pair.iter().map(|s| {
                InlineKeyboardButton::callback(
                    format!("{} {}₽", s.name_service, s.price),
                    format!("service_{}", s.service_id),
                )
            }).collect()
        })
        .collect();

    rows.push(vec![InlineKeyboardButton::callback("⬅️ Назад", "back_to_services")]);
    InlineKeyboardMarkup::new(rows)
}
