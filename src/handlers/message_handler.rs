use teloxide::{prelude::*, utils::command::BotCommands};
use crate::commands::{Command, start::start, help::help, services::services};

pub async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        match Command::parse(text, "union_bot") {
            Ok(Command::Start) => start(bot, msg).await,
            Ok(Command::Help) => help(bot, msg).await,
            Ok(Command::Services) => services(bot, msg).await,
            Err(_) => {
                bot.send_message(msg.chat.id, "Неизвестная команда, нажмите /help для просмотра все доступных команд").await?;
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}
