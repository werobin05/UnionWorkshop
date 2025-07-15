use teloxide::prelude::*;

pub async fn help(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Команды: 
        \n/start - начало работы с ботом;
        \n/help - справка и помощь;
        \n/services - список все доступных услуг;
        \n/orders - список ваших заказов;")
        .await?;
    Ok(())
}