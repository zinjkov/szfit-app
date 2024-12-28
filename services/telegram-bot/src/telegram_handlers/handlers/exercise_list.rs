use crate::sender::{IntoTelegramReply, TelegramReply};
use crate::state::{auth, Screen};
use crate::telegram_handlers::callback_handler::CallbackHandlerProcessor;
use crate::telegram_handlers::error::HandlerError;
use crate::telegram_handlers::handlers::callback_handler::CallbackHandler;
use crate::telegram_handlers::set_state;
use crate::views::{workout_list_view, workout_progress_view, ExerciseViewModel, WorkoutPlanCallbackData, WorkoutPlanCallbackType};
use crate::HandlerResult;
use async_trait::async_trait;
use std::sync::Arc;
use szfit_domain::entity::Id;
use szfit_domain::services::IWorkoutListService;
use szfit_domain::usecases;
use teloxide::dispatching::dialogue::GetChatId;
use tokio::sync::Mutex;

pub struct ExerciseList {}

#[async_trait]
impl CallbackHandler for ExerciseList {
    async fn apply(&self, chp: &CallbackHandlerProcessor) -> HandlerResult<TelegramReply> {
        let data = chp.callback_query.data
            .as_ref()
            .ok_or(HandlerError::EmptyCallbackData)?;
        let data: WorkoutPlanCallbackData = serde_json::from_str(data)?;
        let user = auth(&chp.catalog, chp.callback_query.chat_id().unwrap(), &chp.user_dialog).await?;
        let reply;
        match data.callback_type {
            WorkoutPlanCallbackType::Start => {
                let workout_id: Id = data.workout_id.ok_or(HandlerError::WrongArgs)?.into();
                let mut user_state = chp.user_dialog.get_or_default().await?;
                let exercise_list = chp.catalog.get_one::<dyn IWorkoutListService>()?
                    .list_exercise_for_workout(workout_id).await?;

                let usecase = usecases::WorkoutUsecase::new(
                    user.id,
                    workout_id,
                    chp.catalog.get_one()?,
                    chp.catalog.get_one()?)
                    .await
                    .ok_or(HandlerError::Other)?;

                let exercise_list = exercise_list.into_iter()
                    .map(|e| {
                        ExerciseViewModel {
                            id: e.id,
                            name: e.name,
                            checked: usecase.has_sets(&e.id),
                        }
                    }).collect();
                user_state.workout_usecase = Some(Arc::new(Mutex::new(usecase)));
                chp.user_dialog.update(user_state).await?;

                reply = workout_progress_view(exercise_list).into_reply();

                set_state(&chp.user_dialog, Screen::WorkoutInProgress).await?;
            }
            WorkoutPlanCallbackType::Back => {
                let workout_list = chp.catalog.get_one::<dyn IWorkoutListService>()?
                    .list_workout_for_user(user.id)
                    .await?;

                reply = workout_list_view(workout_list).into_reply();
                set_state(&chp.user_dialog, Screen::WorkoutList).await?;
            }
        }
        Ok(reply)
    }
}