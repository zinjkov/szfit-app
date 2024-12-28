use crate::jwt_auth::claims::Claims;
use crate::jwt_auth::prelude::JwtValidator;
use crate::jwt_auth::JwtSecret;
use crate::services::error::ServiceResult;
use crate::services::AuthClaims;
use dill::component;

pub trait IJwtAuthenticator: Send + Sync {
    fn authenticate(&self, token: &str) -> ServiceResult<AuthClaims>;
}

#[component]
pub struct JwtAuthenticator {
    secret: JwtSecret,
}

impl JwtAuthenticator {
    pub fn new(secret: JwtSecret) -> JwtAuthenticator {
        Self {
            secret
        }
    }
}

impl IJwtAuthenticator for JwtAuthenticator {
    fn authenticate(&self, token: &str) -> ServiceResult<AuthClaims> {
        let claims: Claims<AuthClaims> = JwtValidator::new(self.secret.0.as_str()).validate(token)?;
        Ok(claims.user_claims)
    }
}


#[cfg(test)]
mod test {
    // use crate::jwt_auth::claims::Claims;
    // use crate::jwt_auth::jwt_validator::JwtValidator;
    // use crate::services::AuthClaims;

    #[test]
    fn exctract_user() {
        // let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2NsYWltcyI6eyJ1c2VyX2lkIjoxfSwiZXhwIjoxNzI4NjcxMzE1fQ.YnQsjKqa6iCQIuRJGQ1Swe8NWJxai02wV_Oz6Ha9MDU";
        // let secret = "secret";
        // let claims: Claims<AuthClaims> = JwtValidator::new(secret).validate(token).unwrap();
        // println!("{claims:?}")
    }
}