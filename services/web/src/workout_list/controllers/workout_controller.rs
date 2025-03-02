use crate::{
    app_response::WebResult,
    workout_list::workout_dto::{
        WorkoutDto, WorkoutForCreate, WorkoutForDelete, WorkoutForUpdate,
    },
};
use std::sync::Arc;
use szfit_domain::{entity::Id, services::IWorkoutListService};
use szfit_macro::ControllerFromState;

#[derive(ControllerFromState)]
pub struct WorkoutController {
    workout_service: Arc<dyn IWorkoutListService>,
}

impl WorkoutController {
    #[allow(unused)]
    pub fn new(workout_service: Arc<dyn IWorkoutListService>) -> Self {
        Self { workout_service }
    }
    pub async fn list_workout_for_user(
        &self, user_id: Id,
    ) -> WebResult<Vec<WorkoutDto>> {
        log::info!("list_workout_for_user processing for {}", *user_id);
        Ok(self
            .workout_service
            .list_workout_for_user(user_id)
            .await?
            .into_iter()
            .map(|w| w.into())
            .collect())
    }

    pub async fn create_workout(
        &self, user_id: Id, workout_for_create: WorkoutForCreate,
    ) -> WebResult<WorkoutDto> {
        Ok(self
            .workout_service
            .create_workout(user_id, workout_for_create.name)
            .await?
            .into())
    }

    pub async fn delete_workout(
        &self, workout_for_delete: WorkoutForDelete,
    ) -> WebResult<()> {
        Ok(self
            .workout_service
            .delete_workout(workout_for_delete.id)
            .await?)
    }

    pub async fn update_workout(
        &self, workout_for_update: WorkoutForUpdate,
    ) -> WebResult<WorkoutDto> {
        Ok(self
            .workout_service
            .update_workout(workout_for_update.id, workout_for_update.name)
            .await?
            .into())
    }
}
