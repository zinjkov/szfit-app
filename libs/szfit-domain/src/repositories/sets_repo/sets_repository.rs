use crate::entity::{Id, Sets};
use crate::repositories::error::RepoResult;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct SetsForCreate {
    pub weight_kg: f32,
    pub reps: i64,
    pub user_id: Id,
    pub exercise_id: Id,
    pub training_id: Id,
}

#[async_trait::async_trait]
pub trait ISetsRepository: Send + Sync {
    async fn create(&self, sets: Vec<SetsForCreate>) -> RepoResult<()>;
    async fn last_max_set(
        &self,
        user_id: Id,
        exercise_id: Id,
    ) -> RepoResult<Sets>;
}
