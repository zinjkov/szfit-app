use crate::entity::{Id, User};
use crate::repositories::IUserRepository;
use crate::services::auth_service::auth_service::IAuthService;
use crate::services::error::*;
use async_trait::async_trait;
use dill::component;
use std::sync::Arc;

#[component]
pub struct TelegramAuthService {
    user_repo: Arc<dyn IUserRepository>,
}

impl TelegramAuthService {
    pub fn new(user_repo: Arc<dyn IUserRepository>) -> Self {
        Self{ user_repo }
    }
}

#[async_trait]
impl IAuthService for TelegramAuthService {
    async fn auth_or_create(&self, telegram_id: Id) -> ServiceResult<User> {
        Ok(self.user_repo.find_by_tg_or_create(telegram_id).await?)
    }

    async fn auth(&self, telegram_id: Id) -> ServiceResult<User> {
        Ok(self.user_repo.find_by_tg(telegram_id).await?)
    }
}