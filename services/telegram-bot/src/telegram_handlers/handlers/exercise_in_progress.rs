use crate::sender::{IntoTelegramReply, TelegramReply};
use crate::state::Screen;
use crate::telegram_handlers::callback_handler::CallbackHandlerProcessor;
use crate::telegram_handlers::error::{HandlerError, HandlerResult};
use crate::telegram_handlers::handlers::callback_handler::CallbackHandler;
use crate::telegram_handlers::set_state;
use crate::views::{workout_progress_view, ExerciseViewModel};
use async_trait::async_trait;
use szfit_domain::services::IWorkoutListService;

pub struct ExerciseInProgress {}

#[async_trait]
impl CallbackHandler for ExerciseInProgress {
    async fn apply(
        &self,
        chp: &CallbackHandlerProcessor,
    ) -> HandlerResult<TelegramReply> {
        let data = chp
            .callback_query
            .data
            .as_ref()
            .ok_or(HandlerError::EmptyCallbackData)?;
        if data != "stop_exercise" {
            return Err(HandlerError::EmptyCallbackData);
        }
        let user_state = chp.user_dialog.get_or_default().await?;
        let usecase =
            user_state.workout_usecase.as_ref().unwrap().lock().await;
        let id = usecase.current_workout();
        let exercise_list = chp
            .catalog
            .get_one::<dyn IWorkoutListService>()?
            .list_exercise_for_workout(id)
            .await?;

        let exercise_list = exercise_list
            .into_iter()
            .map(|e| ExerciseViewModel {
                id: e.id,
                name: e.name,
                checked: usecase.has_sets(&e.id),
            })
            .collect();

        let view = workout_progress_view(exercise_list);
        set_state(&chp.user_dialog, Screen::WorkoutInProgress).await?;
        Ok(view.into_reply())
    }
}
