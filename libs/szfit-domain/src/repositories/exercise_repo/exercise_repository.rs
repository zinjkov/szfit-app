use crate::entity::{Exercise, Id};
use crate::repositories::error::RepoResult;

pub struct ExerciseForCreate {
    pub name: String,
}

#[async_trait::async_trait]
pub trait IExerciseRepository: Send + Sync {
    async fn find_by_id(&self, exercise_id: Id) -> RepoResult<Exercise>;
    async fn list(
        &self,
        limit: usize,
        offset: usize,
    ) -> RepoResult<Vec<Exercise>>;
    async fn list_default(&self) -> RepoResult<Vec<Exercise>> {
        self.list(self.default_limit(), 0).await
    }

    async fn list_with_limit(
        &self,
        limit: usize,
    ) -> RepoResult<Vec<Exercise>> {
        self.list(limit, 0).await
    }

    async fn create(
        &self,
        exercise_for_insert: ExerciseForCreate,
    ) -> RepoResult<Exercise>;

    fn default_limit(&self) -> usize {
        20
    }
}
