use crate::sender::{IntoTelegramReply, TelegramReply};
use crate::state::Screen;
use crate::telegram_handlers::callback_handler::CallbackHandlerProcessor;
use crate::telegram_handlers::error::{HandlerError, HandlerResult};
use crate::telegram_handlers::handlers::callback_handler::CallbackHandler;
use crate::telegram_handlers::set_state;
use crate::views::workout_plan_view;
use async_trait::async_trait;
use std::sync::Arc;
use szfit_domain::entity::Id;
use szfit_domain::services::IWorkoutPlanService;

pub struct WorkoutListHandler {}

#[async_trait]
impl CallbackHandler for WorkoutListHandler {
    async fn apply(
        &self,
        chp: &CallbackHandlerProcessor,
    ) -> HandlerResult<TelegramReply> {
        let plan = {
            let data = chp
                .callback_query
                .data
                .as_ref()
                .ok_or(HandlerError::EmptyCallbackData)?;
            let workout_id: i64 = serde_json::from_str(data)?;
            let service: Arc<dyn IWorkoutPlanService> =
                chp.catalog.get_one()?;
            service.get_workout_plan(Id(workout_id)).await?
        };

        let reply = workout_plan_view(plan).into_reply();
        set_state(&chp.user_dialog, Screen::ExerciseList).await?;
        Ok(reply)
    }
}
