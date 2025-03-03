use crate::{
    app_response::{IntoJsonResult, JsonResult},
    workout_edit::{
        controllers::workout_plan_controller::WorkoutEditController,
        workout_dto::{SetExerciseWorkout, WorkoutDto},
    },
};
use axum::Json;

pub(super) async fn update_exercise_into_workout(
    workout_controller: WorkoutEditController,
    Json(exercise_workout): Json<SetExerciseWorkout>,
) -> JsonResult<WorkoutDto> {
    log::debug!(
        "workout_id: {:?}, exercise_ids: {:?}",
        exercise_workout.workout_id,
        exercise_workout.exercise_ids
    );

    workout_controller
        .set_exercises(exercise_workout)
        .await
        .into_json()
}
