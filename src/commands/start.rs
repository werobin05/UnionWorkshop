use teloxide::prelude::*;

pub async fn start(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Добро пожаловать! Рады вас видеть. 
    Для того что бы сделать заказ нажмите на /services
    Все доступные команды можно просмотреть нажав: /help
    \n
    Если возникли вопросы пишите нам на электронную почту")
        .await?;
    Ok(())
}