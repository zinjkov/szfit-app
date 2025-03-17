use crate::jwt_auth::claims::Claims;
use crate::jwt_auth::prelude::JwtValidator;
use crate::jwt_auth::JwtSecret;
use crate::services::error::ServiceResult;
use crate::services::jwt_service::{AuthClaims, IJwtAuthenticator};
use dill::component;

#[component]
pub struct JwtAuthenticator {
    secret: JwtSecret,
}

#[cfg(test)]
impl JwtAuthenticator {
    #[allow(unused)]
    pub fn new(secret: JwtSecret) -> JwtAuthenticator {
        Self {
            secret,
        }
    }
}

impl IJwtAuthenticator for JwtAuthenticator {
    fn authenticate(&self, token: &str) -> ServiceResult<AuthClaims> {
        let claims: Claims<AuthClaims> =
            JwtValidator::new(self.secret.as_str()).validate(token)?;
        Ok(claims.user_claims)
    }
}
