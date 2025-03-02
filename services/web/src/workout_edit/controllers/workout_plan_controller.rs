use crate::{
    app_response::WebResult,
    workout_edit::workout_dto::{SetExerciseWorkout, WorkoutDto},
};
use std::sync::Arc;
use szfit_domain::services::IWorkoutPlanService;
use szfit_macro::ControllerFromState;

#[derive(ControllerFromState)]
pub struct WorkoutEditController {
    workout_service: Arc<dyn IWorkoutPlanService>,
}

impl WorkoutEditController {
    pub async fn set_exercises(
        &self, exercise_workout: SetExerciseWorkout,
    ) -> WebResult<WorkoutDto> {
        Ok(self
            .workout_service
            .set_exercises(
                exercise_workout.workout_id,
                exercise_workout.exercise_ids,
            )
            .await?
            .into())
    }
}
