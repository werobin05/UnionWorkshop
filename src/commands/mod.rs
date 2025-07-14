use teloxide::utils::command::BotCommands;

pub mod start;
pub mod help;
pub mod services;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description ="список команд")]
pub enum Command {
    #[command(description = "Начать работу")]
    Start,
    #[command(description = "Справка и помощь")]
    Help,
    #[command(description = "Список заказов")]
    Services,
}