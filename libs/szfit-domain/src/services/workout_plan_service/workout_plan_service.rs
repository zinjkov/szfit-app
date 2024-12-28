use async_trait::async_trait;
use crate::aggregate::WorkoutPlan;
use crate::entity::Id;
use crate::services::error::ServiceResult;

#[async_trait]
pub trait IWorkoutPlanService: Send + Sync {
    async fn get_workout_plan(&self, workout_id: Id) -> ServiceResult<WorkoutPlan>;

    async fn set_exercises(&self, workout_id: Id, exercise_ids: Vec<Id>) -> ServiceResult<WorkoutPlan>;
}