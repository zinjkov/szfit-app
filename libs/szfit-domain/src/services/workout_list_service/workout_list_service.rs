use crate::entity::{Exercise, Id, Workout};
use crate::services::error::ServiceResult;
use async_trait::async_trait;

#[async_trait]
pub trait IWorkoutListService: Send + Sync {
    async fn create_workout(
        &self,
        user_id: Id,
        name: String,
    ) -> ServiceResult<Workout>;

    async fn delete_workout(&self, user_id: Id) -> ServiceResult<()>;

    async fn update_workout(
        &self,
        workout_id: Id,
        name: String,
    ) -> ServiceResult<Workout>;

    async fn list_workout_for_user(
        &self,
        user_id: Id,
    ) -> ServiceResult<Vec<Workout>>;

    async fn list_exercise_for_workout(
        &self,
        workout_id: Id,
    ) -> ServiceResult<Vec<Exercise>>;
}
