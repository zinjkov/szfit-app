use axum::Json;
use serde::{Deserialize, Serialize};
use szfit_domain::aggregate::Tokens;

use crate::{
    app_response::{IntoJsonResult, JsonResult},
    auth::telegram_auth_controller::TelegramAuthController,
};

pub mod jwt;
mod telegram_auth_controller;

#[derive(Debug, Deserialize)]
pub struct TelegramAuth {
    telegram_id: i64,
}

#[derive(Serialize)]
#[serde(rename = "Tokens")]
pub struct TokensDto {
    access_token: String,
    refresh_token: String,
}

impl From<Tokens> for TokensDto {
    fn from(value: Tokens) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
        }
    }
}

pub async fn telegram_auth(
    controller: TelegramAuthController, Json(telegram_auth): Json<TelegramAuth>,
) -> JsonResult<TokensDto> {
    log::info!("telegram_auth {}", telegram_auth.telegram_id);
    controller.auth(telegram_auth).await.into_json()
}
