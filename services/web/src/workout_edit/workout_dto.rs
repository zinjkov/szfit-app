use serde::{Deserialize, Serialize};
use szfit_domain::{
    aggregate::WorkoutPlan,
    entity::{Exercise, Id},
};
use utoipa::ToSchema;

#[derive(Deserialize)]
pub struct SetExerciseWorkout {
    pub workout_id: Id,
    pub exercise_ids: Vec<Id>,
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
        Self { id: Some(value.id), name: value.name }
    }
}

impl From<WorkoutPlan> for WorkoutDto {
    fn from(value: WorkoutPlan) -> Self {
        Self {
            workout_id: value.workout_id,
            workout_name: value.workout_name,
            exercises: value
                .exercise_list
                .into_iter()
                .map(ExerciseDto::from)
                .collect(),
        }
    }
}
