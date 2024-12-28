use async_trait::async_trait;
use szfit_domain::entity::Id;
use szfit_domain::repositories;
use crate::sender::{IntoTelegramReply, TelegramReply};
use crate::state::Screen;
use crate::telegram_handlers::callback_handler::CallbackHandlerProcessor;
use crate::telegram_handlers::error::{HandlerError, HandlerResult};
use crate::telegram_handlers::handlers::callback_handler::CallbackHandler;
use crate::telegram_handlers::set_state;
use crate::views::exercise_progress_view;

pub struct WorkoutInProgressHandler;

#[async_trait]
impl CallbackHandler for WorkoutInProgressHandler {
    async fn apply(&self, chp: &CallbackHandlerProcessor) -> HandlerResult<TelegramReply> {
        let mut us = chp.user_dialog.get_or_default().await?;
        let data = chp.callback_query.data.as_ref().ok_or(HandlerError::EmptyCallbackData)?;
        let usecase = us.workout_usecase.as_ref().ok_or(HandlerError::WrongState)?.clone();
        if data == "stop_workout" {
            usecase.lock()
                .await
                .finish_workout()
                .await
                .ok_or(HandlerError::Other)?;
            us.workout_usecase = None;
            chp.user_dialog.update(us).await?;
            return Ok("Тренировка закончена! 🤙🏿".to_owned().into_reply());
        }
        let exercise_id: Id = serde_json::from_str::<i64>(data)?.into();

        usecase.lock()
            .await
            .start_exercise(exercise_id);

        let exercise_repo = chp.catalog.get_one::<dyn repositories::IExerciseRepository>()?;
        let exercise_name = exercise_repo.find_by_id(exercise_id).await?.name;
        let max_last_set = usecase.lock()
            .await.last_max_set().await;
        let reply = exercise_progress_view(exercise_name, max_last_set).into_reply();

        let _ = set_state(&chp.user_dialog, Screen::ExerciseInProgress)
            .await;
        Ok(reply)
    }
}