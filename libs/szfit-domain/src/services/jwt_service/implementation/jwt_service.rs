use crate::aggregate::Tokens;
use crate::entity::User;
use crate::jwt_auth::prelude::*;
use crate::jwt_auth::JwtSecret;
use crate::services::error::ServiceResult;
use crate::services::jwt_service::auth_claims::AuthClaims;
use crate::services::IJwtService;
use async_trait::async_trait;
use chrono::Duration;
use dill::component;
use std::ops::Deref;

#[component]
pub struct JwtService {
    secret: JwtSecret,
    access_token_exp_time: AccessTokenExpTime,
    refresh_token_exp_time: RefreshTokenExpTime,
}

#[derive(Clone)]
pub struct AccessTokenExpTime(pub Duration);

impl Deref for AccessTokenExpTime {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct RefreshTokenExpTime(pub Duration);

impl Deref for RefreshTokenExpTime {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl IJwtService for JwtService {
    async fn create_tokens(&self, user: User) -> ServiceResult<Tokens> {
        let claims = AuthClaims::from(&user);

        let tokens = Tokens {
            access_token: self.new_access_token(claims.clone())?,
            refresh_token: self.new_refresh_token(claims)?,
        };

        Ok(tokens)
    }

    async fn refresh(&self, refresh_token: &str) -> ServiceResult<String> {
        let user = self.user_claims_from(refresh_token)?;
        let new_token = self.new_access_token(user)?;

        Ok(new_token)
    }
}

impl JwtService {
    pub fn user_claims_from(
        &self,
        token: &str,
    ) -> ServiceResult<AuthClaims> {
        let claims: AuthClaims =
            JwtValidator::new(self.secret.0.as_str()).validate(token)?;
        Ok(claims)
    }

    pub fn new_access_token(
        &self,
        claims: impl Into<AuthClaims>,
    ) -> ServiceResult<String> {
        self.new_token(claims, *self.access_token_exp_time)
    }

    pub fn new_refresh_token(
        &self,
        claims: impl Into<AuthClaims>,
    ) -> ServiceResult<String> {
        self.new_token(claims, *self.refresh_token_exp_time)
    }

    pub fn new_token(
        &self,
        claims: impl Into<AuthClaims>,
        expiration: Duration,
    ) -> ServiceResult<String> {
        let claims = claims.into();
        let token = JwtGenerator::new(self.secret.0.as_str(), expiration)
            .generate(claims)?;
        Ok(token)
    }
}
