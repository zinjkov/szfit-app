use crate::entity::{Id, User};
use crate::services::error::*;
use async_trait::async_trait;

#[async_trait]
pub trait IAuthService: Send + Sync {
    async fn auth_or_create(&self, telegram_id: Id)
        -> ServiceResult<User>;
    async fn auth(&self, telegram_id: Id) -> ServiceResult<User>;
}
