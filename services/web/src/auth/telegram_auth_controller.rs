use crate::{
    app_response::WebResult,
    auth::{TelegramAuth, TokensDto},
};
use std::sync::Arc;
use szfit_domain::services::{IAuthService, IJwtService};
use szfit_macro::ControllerFromState;

#[derive(ControllerFromState)]
pub struct TelegramAuthController {
    auth_service: Arc<dyn IAuthService>,
    jwt_service: Arc<dyn IJwtService>,
}

impl TelegramAuthController {
    pub async fn auth(
        &self, telegram_auth: TelegramAuth,
    ) -> WebResult<TokensDto> {
        let user = self
            .auth_service
            .auth_or_create(telegram_auth.telegram_id.into())
            .await?;
        let tokens = self.jwt_service.create_tokens(user).await?;
        Ok(TokensDto::from(tokens))
    }
}
