use crate::aggregate::WorkoutPlan;
use crate::entity::{Exercise, Id, Workout};
use crate::repositories::error::RepoResult;
use async_trait::async_trait;

#[async_trait]
pub trait IWorkoutPlanRepository: Send + Sync {
    fn default_limit(&self) -> usize {
        20
    }

    async fn list_with_limit_offset(
        &self,
        user_id: Id,
        limit: usize,
        offset: usize,
    ) -> RepoResult<Vec<Workout>>;

    async fn list_with_limit(
        &self,
        user_id: Id,
        limit: usize,
    ) -> RepoResult<Vec<Workout>> {
        self.list_with_limit_offset(user_id, limit, 0).await
    }

    async fn list(&self, user_id: Id) -> RepoResult<Vec<Workout>> {
        self.list_with_limit(user_id, self.default_limit()).await
    }

    async fn create(
        &self,
        user_id: Id,
        workout_name: String,
    ) -> RepoResult<Workout>;

    async fn delete(&self, workout_id: Id) -> RepoResult<()>;

    async fn update(
        &self,
        workout_id: Id,
        workout_name: String,
    ) -> RepoResult<Workout>;

    async fn find_by_id(&self, workout_id: Id) -> RepoResult<WorkoutPlan>;

    async fn add_exercises(
        &self,
        workout_id: Id,
        exercise_ids: Vec<Id>,
    ) -> RepoResult<WorkoutPlan>;

    async fn exercises_for_workout(
        &self,
        workout_id: Id,
    ) -> RepoResult<Vec<Exercise>>;
}
