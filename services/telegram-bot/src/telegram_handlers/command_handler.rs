use std::collections::HashMap;
use dill::Catalog;
use teloxide::Bot;
use teloxide::types::Message;

use szfit_domain::services::IWorkoutListService;

use crate::sender::{send_text, send_text_with_button};
use crate::state::{auth, Screen, UserDialogue};
use crate::telegram_command::TelegramCommand;
use crate::telegram_handlers::set_state;
use crate::views::workout_list_view;

use super::error::*;

pub struct CommandHandlerProcessor {
    bot: Bot,
    command: TelegramCommand,
    message: Message,
    user_dialog: UserDialogue,
    catalog: Catalog,
    process_handlers: HashMap<TelegramCommand, Box<dyn ProcessHandler>>
}

impl CommandHandlerProcessor {
    pub fn new(bot: Bot, command: TelegramCommand, message: Message, user_dialog: UserDialogue, catalog: Catalog) -> Self {
        Self { bot, command, message, user_dialog, catalog , process_handlers: Self::make_handlers()}
    }
    pub async fn process(self) -> HandlerResult<()> {
        let handler = self.process_handlers
            .get(&self.command)
            .ok_or(HandlerError::WrongArgs)?;
        handler.process(&self).await?;
        Ok(())
    }

    fn make_handlers() -> HashMap<TelegramCommand, Box<dyn ProcessHandler>> {
        let mut handlers = HashMap::<TelegramCommand, Box<dyn ProcessHandler>>::new();
        handlers.insert(TelegramCommand::Start, Box::new(StartHandler{}));
        handlers.insert(TelegramCommand::WhoAmI, Box::new(WhoAmIHandler{}));
        handlers
    }
}

async fn handle_start(bot: &Bot, message: &Message, user_dialog: &UserDialogue, catalog: &Catalog) -> HandlerResult<()> {
    let user = auth(&catalog, message.chat.id, &user_dialog).await?;
    let workout_list = catalog.get_one::<dyn IWorkoutListService>()?
        .list_workout_for_user(user.id)
        .await?;

    let (text, keyboard) = workout_list_view(workout_list);
    let result = send_text_with_button(&bot, message.chat.id, text, keyboard)
        .await
        .map(|_| ())?;

    set_state(&user_dialog, Screen::WorkoutList).await?;

    Ok(result)
}

#[async_trait::async_trait]
trait ProcessHandler: Send + Sync {
    async fn process(&self, processor: &CommandHandlerProcessor) -> HandlerResult<()>;
}

struct StartHandler {}
#[async_trait::async_trait]
impl ProcessHandler for StartHandler {
    async fn process(&self, processor: &CommandHandlerProcessor) -> HandlerResult<()> {
        handle_start(&processor.bot, &processor.message, &processor.user_dialog, &processor.catalog).await
    }
}

struct WhoAmIHandler {}
#[async_trait::async_trait]
impl ProcessHandler for WhoAmIHandler {
    async fn process(&self, processor: &CommandHandlerProcessor) -> HandlerResult<()> {
        send_text(&processor.bot, processor.message.chat.id, processor.message.chat.id.0.to_string()).await?;
        Ok(())
    }
}