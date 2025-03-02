use crate::aggregate::Tokens;
use crate::entity::User;
use crate::services::error::ServiceResult;
use async_trait::async_trait;

#[async_trait]
pub trait IJwtService: Send + Sync {
    async fn create_tokens(&self, user: User) -> ServiceResult<Tokens>;
    async fn refresh(&self, refresh_token: &str) -> ServiceResult<String>;
}
