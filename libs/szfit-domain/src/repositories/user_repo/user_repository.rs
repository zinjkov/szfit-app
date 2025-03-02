use crate::entity::{Id, User};
use crate::repositories::error::RepoResult;

#[async_trait::async_trait]
pub trait IUserRepository: Send + Sync {
    async fn find_by_tg(&self, telegram_id: Id) -> RepoResult<User>;
    async fn find_by_id(&self, id: Id) -> RepoResult<User>;
    async fn find_by_tg_or_create(
        &self,
        telegram_id: Id,
    ) -> RepoResult<User>;
    async fn create(&self, telegram_id: Id) -> RepoResult<User>;
}
