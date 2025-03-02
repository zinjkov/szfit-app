use dill::Catalog;
use std::collections::HashMap;
use teloxide::prelude::CallbackQuery;
use teloxide::Bot;

use super::error::*;
use crate::sender::TelegramSend;
use crate::state::{Screen, UserDialogue};
use crate::telegram_handlers::handlers::callback_handler::CallbackHandler;
use crate::telegram_handlers::handlers::exercise_in_progress::ExerciseInProgress;
use crate::telegram_handlers::handlers::exercise_list::ExerciseList;
use crate::telegram_handlers::handlers::workout_in_progress::WorkoutInProgressHandler;
use crate::telegram_handlers::handlers::workout_list_handler::WorkoutListHandler;

pub struct CallbackHandlerProcessor {
    bot: Bot,
    pub(crate) callback_query: CallbackQuery,
    pub(crate) user_dialog: UserDialogue,
    pub(crate) catalog: Catalog,
    screen_handlers: HashMap<Screen, Box<dyn CallbackHandler>>,
}

fn handlers() -> HashMap<Screen, Box<dyn CallbackHandler>> {
    let mut handlers: HashMap<Screen, Box<dyn CallbackHandler>> =
        HashMap::new();
    handlers.insert(Screen::WorkoutList, Box::new(WorkoutListHandler {}));
    handlers.insert(Screen::ExerciseList, Box::new(ExerciseList {}));
    handlers.insert(
        Screen::WorkoutInProgress,
        Box::new(WorkoutInProgressHandler {}),
    );
    handlers.insert(
        Screen::ExerciseInProgress,
        Box::new(ExerciseInProgress {}),
    );
    handlers
}

impl CallbackHandlerProcessor {
    pub fn new(
        bot: Bot,
        callback_query: CallbackQuery,
        user_dialog: UserDialogue,
        catalog: Catalog,
    ) -> Self {
        Self {
            bot,
            callback_query,
            user_dialog,
            catalog,
            screen_handlers: handlers(),
        }
    }
    pub async fn process(self) -> HandlerResult<()> {
        let screen = self.current_screen().await?;
        log::info!("Current screen: {:?}", screen);
        let _ = self
            .screen_handlers
            .get(&screen)
            .ok_or(HandlerError::CallbackNotFound)
            .inspect_err(|err| {
                log::error!("callback not found: {:?}", err)
            })?
            .apply(&self)
            .await?
            .send_message(&self.bot, self.user_dialog.chat_id())
            .await?;
        Ok(())
    }

    async fn current_screen(&self) -> HandlerResult<Screen> {
        Ok(self.user_dialog.get_or_default().await?.state)
    }
}
