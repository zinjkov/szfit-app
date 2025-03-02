use crate::services::auth_service::auth_service::IAuthService;
use crate::services::auth_service::implementation::telegram_auth_service::TelegramAuthService;
use dill::CatalogBuilder;

pub mod auth_service;
mod implementation;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder
        .add::<TelegramAuthService>()
        .bind::<dyn IAuthService, TelegramAuthService>();
}
