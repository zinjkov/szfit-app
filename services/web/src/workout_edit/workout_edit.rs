use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use dill::Catalog;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use szfit_domain::aggregate::WorkoutPlan;
use szfit_domain::entity::{Exercise, Id};
use szfit_domain::services::{IWorkoutPlanService};
use crate::app_response::JsonResult;

#[derive(Deserialize)]
pub struct SetExerciseWorkout {
    workout_id: Id,
    exercise_ids: Vec<Id>,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct WorkoutDto {
    pub workout_id: Id,
    pub workout_name: String,
    pub exercises: Vec<ExerciseDto>,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct ExerciseDto {
    id: Option<Id>,
    name: String,
}

impl From<Exercise> for ExerciseDto {
    fn from(value: Exercise) -> Self {
        Self {
            id: Some(value.id),
            name: value.name,
        }
    }
}

impl From<WorkoutPlan> for WorkoutDto {
    fn from(value: WorkoutPlan) -> Self {
        Self {
            workout_id: value.workout_id,
            workout_name: value.workout_name,
            exercises: value.exercise_list.into_iter().map(ExerciseDto::from).collect(),
        }
    }
}

pub(super) async fn update_exercise_into_workout(State(catalog): State<Catalog>,
                                    Json(exercise_workout): Json<SetExerciseWorkout>)
                                    -> JsonResult<WorkoutDto> {
    log::debug!("workout_id: {:?}, exercise_ids: {:?}", exercise_workout.workout_id, exercise_workout.exercise_ids);
    let workout_service = catalog.get_one::<dyn IWorkoutPlanService>().unwrap();
    let result = workout_service
        .set_exercises(exercise_workout.workout_id, exercise_workout.exercise_ids)
        .await?;

    Ok((StatusCode::CREATED, Json(result.into())))
}