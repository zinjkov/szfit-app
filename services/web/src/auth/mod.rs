use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use dill::Catalog;
use serde::{Deserialize, Serialize};
use szfit_domain::aggregate::Tokens;

use szfit_domain::services;
use crate::app_response::{JsonResult};
use crate::auth::jwt::AuthError;

pub mod jwt;

#[derive(Debug, Deserialize)]
pub struct TelegramAuth {
    telegram_id: String,
}

#[derive(Serialize)]
#[serde(rename="Tokens")]
pub struct TokensDto {
    access_token: String,
    refresh_token: String
}

impl From<Tokens> for TokensDto {
    fn from(value: Tokens) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
        }
    }
}

pub async fn telegram_auth(State(catalog): State<Catalog>, Json(telegram_auth): Json<TelegramAuth>)
                       -> JsonResult<TokensDto>  {
    log::info!("telegram_auth {}", telegram_auth.telegram_id);
    let auth_service = catalog.get_one::<services::AuthService>()?;
    let id: i64 = telegram_auth.telegram_id.parse().map_err(|_| AuthError::WrongCredentials)?;
    let user = auth_service.user_or_create(id.into()).await?;
    let auth_service = catalog.get_one::<dyn services::IJwtService>()?;
    let tokens = auth_service.create_tokens(user).await?;
    Ok((StatusCode::OK, Json(TokensDto::from(tokens))))
}