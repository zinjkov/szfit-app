use crate::entity::{Id, Training};
use crate::repositories::error::RepoResult;

pub struct TrainingForCreate {
    pub name: Option<String>,
    pub user_id: Id,
    pub workout_plan_id: Id,
}

#[derive(Default)]
pub struct TrainingForUpdate {
    pub name: Option<String>,
    pub workout_plan_id: Option<i32>,
    pub finished_at: Option<chrono::NaiveDateTime>,
}
#[async_trait::async_trait]
pub trait ITrainingRepository: Send + Sync {
    async fn create(&self, tfc: TrainingForCreate) -> RepoResult<Training>;
    async fn list(&self, user_id: Id, limit: usize, offset: usize) -> RepoResult<Vec<Training>>;
    async fn list_default(&self, user_id: Id) -> RepoResult<Vec<Training>> {
        self.list(user_id, self.default_limit(), 0).await
    }
    async fn list_with_limit(&self, user_id: Id, limit: usize) -> RepoResult<Vec<Training>> {
        self.list(user_id, limit, 0).await
    }
    async fn list_with_offset(&self, user_id: Id, offset: usize) -> RepoResult<Vec<Training>> {
        self.list(user_id, self.default_limit(), offset).await
    }
    async fn update(&self, training_id: Id, tfu: TrainingForUpdate) -> RepoResult<Training>;

    fn default_limit(&self) -> usize {
        30
    }
}