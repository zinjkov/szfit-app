use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone, PartialEq, Eq, Hash)]
#[command(description = "Commands:", rename_rule = "lowercase")]
pub enum TelegramCommand {
    Start,
    WhoAmI,
}
