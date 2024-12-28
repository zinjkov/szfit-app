use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;

use crate::jwt_auth::claims::Claims;
use crate::jwt_auth::error::JwtAuthResult;
use crate::jwt_auth::jwt_coder::JwtCoder;
pub struct JwtGenerator {
    coder: JwtCoder<EncodingKey, (), Header, ()>,
    expiration_lifetime: Duration
}

impl JwtGenerator {
    pub fn new(secret: &str, expiration_lifetime: Duration) -> Self {
        Self {
            coder: JwtCoder::new_encoder(
                EncodingKey::from_secret(secret.as_bytes()),
                Header::default(),
            ),
            expiration_lifetime
        }
    }

    pub fn generate<UserClaims: Serialize>(&self, user_claims: UserClaims) -> JwtAuthResult<String> {
        let claims = Claims {
            user_claims,
            exp: (Utc::now() + self.expiration_lifetime).timestamp(),
        };
        self.coder.encode(&claims)
    }
}