use crate::sender::send_text;
use szfit_domain::aggregate::Sets;
use teloxide::types::Message;
use teloxide::Bot;

use crate::state::{Screen, UserDialogue};

use super::error::*;

pub struct MessageHandlerProcessor {
    bot: Bot,
    message: Message,
    user_dialog: UserDialogue,
}

impl MessageHandlerProcessor {
    pub fn new(
        bot: Bot,
        message: Message,
        user_dialog: UserDialogue,
    ) -> Self {
        Self {
            bot,
            message,
            user_dialog,
        }
    }
    pub async fn process(self) -> HandlerResult<()> {
        let _ = self.bot;
        let _ = self.message;
        let user_state = self.user_dialog.get_or_default().await?;
        match user_state.state {
            Screen::ExerciseInProgress => {
                self.process_exercise_in_progress().await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn process_exercise_in_progress(&self) -> HandlerResult<()> {
        let usecase = self
            .user_dialog
            .get_or_default()
            .await?
            .workout_usecase
            .ok_or(HandlerError::WrongState)?;
        let mut usecase_lock = usecase.lock().await;
        let sets: Sets = self
            .message
            .text()
            .ok_or(HandlerError::WrongArgs)?
            .parse()
            .map_err(|_| HandlerError::WrongArgs)?;

        usecase_lock.push_sets(sets).await;
        send_text(
            &self.bot,
            self.user_dialog.chat_id(),
            "Записали,отдых ✅".to_owned(),
        )
        .await?;
        let bot = self.bot.clone();
        let chat_id = self.message.chat.id;
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(180))
                .await;
            let _ = send_text(
                &bot,
                chat_id,
                "3 минуты вышло, ебошъ".to_owned(),
            )
            .await;
        });
        Ok(())
    }
}
