use teloxide::prelude::*;

pub async fn orders(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Данная команда находится на переработке").await?;
    Ok(())
}