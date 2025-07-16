use teloxide::prelude::*;
use teloxide::types::{Message, ParseMode};

pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    let help_text = r#"
<b>📖 Доступные команды:</b>

🚀 <b>/start</b> – начать работу с ботом  
🆘 <b>/help</b> – справка и помощь  
🛠️ <b>/services</b> – список доступных услуг  
📦 <b>/orders</b> – список ваших заказов  

💬 По вопросам поддержки просто напишите сообщение.
"#;

    bot.send_message(msg.chat.id, help_text)
        .parse_mode(ParseMode::Html)
        .await?;

    Ok(())
}
