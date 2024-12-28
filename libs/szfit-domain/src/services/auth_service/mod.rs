use dill::CatalogBuilder;
use crate::services::auth_service::auth_service::IAuthService;
use crate::services::auth_service::implementation::telegram_auth_service::TelegramAuthService;

mod implementation;
pub mod auth_service;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder.add::<TelegramAuthService>()
        .bind::<dyn IAuthService, TelegramAuthService>();
}