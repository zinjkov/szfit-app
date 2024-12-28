
use chrono::Duration;
use dill::component;
use serde::{Deserialize, Serialize};

use crate::aggregate::Tokens;
use crate::entity::{Id, User};
use crate::jwt_auth::JwtSecret;
use crate::jwt_auth::prelude::{JwtGenerator, JwtValidator};
use crate::services::error::{ServiceResult};

#[component]
pub struct JwtService {
    secret: JwtSecret,
    access_token_exp_time: AccessTokenExpTime,
    refresh_token_exp_time: RefreshTokenExpTime,
}

#[derive(Clone)]
pub struct AccessTokenExpTime(pub Duration);

#[derive(Clone)]
pub struct RefreshTokenExpTime(pub Duration);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthClaims {
    pub user_id: Id,
}


impl From<&User> for AuthClaims {
    fn from(value: &User) -> Self {
        Self {
            user_id: value.id
        }
    }
}

impl JwtService {
    pub fn user_claims_from(&self, token: &str) -> ServiceResult<AuthClaims> {
        let claims: AuthClaims = JwtValidator::new(self.secret.0.as_str()).validate(token)?;
        Ok(claims)
    }

    pub fn new_access_token(&self, claims: impl Into<AuthClaims>) -> ServiceResult<String> {
        self.new_token(claims, self.access_token_exp_time.0)
    }

    pub fn new_refresh_token(&self, claims: impl Into<AuthClaims>) -> ServiceResult<String> {
        self.new_token(claims, self.refresh_token_exp_time.0)
    }

    pub fn new_token(&self, claims: impl Into<AuthClaims>, expiration: Duration) -> ServiceResult<String> {
        let claims = claims.into();
        let token = JwtGenerator::new(
            self.secret.0.as_str(),
            expiration)
            .generate(claims)?;
        Ok(token)
    }

    pub async fn refresh(&self, access_token: &str) -> ServiceResult<String> {
        let user = self.user_claims_from(access_token)?;
        let new_token = self.new_access_token(user)?;

        Ok(new_token)
    }

    pub async fn create_tokens(&self, user: User) -> ServiceResult<Tokens> {
        let claims = AuthClaims::from(&user);

        let tokens = Tokens {
            access_token: self.new_access_token(claims.clone())?,
            refresh_token: self.new_refresh_token(claims)?,
        };

        Ok(tokens)
    }
}