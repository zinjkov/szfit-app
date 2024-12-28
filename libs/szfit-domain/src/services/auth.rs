use std::sync::Arc;
use async_trait::async_trait;
use dill::component;
use crate::entity::{Id, User};
use crate::repositories::IUserRepository;
use crate::services::error::*;

#[async_trait]
pub trait IAuthService: Send + Sync {

}
#[component]
pub struct AuthService {
    user_repo: Arc<dyn IUserRepository>,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn IUserRepository>) -> Self {
        Self{ user_repo }
    }
    pub async fn user_or_create(&self, telegram_id: Id) -> ServiceResult<User> {
        Ok(self.user_repo.find_by_tg_or_create(telegram_id).await?)
    }

    pub async fn user(&self, telegram_id: Id) -> ServiceResult<User> {
        Ok(self.user_repo.find_by_tg(telegram_id).await?)
    }

}

#[async_trait]
impl IAuthService for AuthService {

}