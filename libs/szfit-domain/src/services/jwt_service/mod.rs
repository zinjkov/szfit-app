use crate::jwt_auth::JwtSecret;
use crate::services::jwt_service::implementation::jwt_authenticator::JwtAuthenticator;
use crate::services::jwt_service::implementation::jwt_service::{
    AccessTokenExpTime, JwtService, RefreshTokenExpTime,
};
pub use auth_claims::*;
use chrono::Duration;
use dill::CatalogBuilder;
pub use jwt_authenticator::*;
pub use jwt_service::*;

pub mod auth_claims;
mod implementation;
pub mod jwt_authenticator;
pub mod jwt_service;

pub fn register_in_catalog(builder: &mut CatalogBuilder) {
    builder.add_value(JwtSecret(String::from("secret")));
    builder.add_value(AccessTokenExpTime(Duration::days(30)));
    builder.add_value(RefreshTokenExpTime(Duration::days(120)));
    builder
        .add::<JwtAuthenticator>()
        .bind::<dyn IJwtAuthenticator, JwtAuthenticator>();
    builder.add::<JwtService>().bind::<dyn IJwtService, JwtService>();
}
