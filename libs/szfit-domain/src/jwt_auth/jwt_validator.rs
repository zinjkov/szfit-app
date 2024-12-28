use jsonwebtoken::{DecodingKey, Validation};
use serde::de::DeserializeOwned;

use crate::jwt_auth::error::JwtAuthResult;
use crate::jwt_auth::jwt_coder::JwtCoder;

pub struct JwtValidator {
    coder: JwtCoder<(), DecodingKey, (), Validation>,
}

impl JwtValidator {
    pub fn new(secret: &str) -> Self {
        Self {
            coder: JwtCoder::new_decoder(
                DecodingKey::from_secret(secret.as_bytes()),
                Validation::default(),
            ),
        }
    }

    pub fn validate<UserClaims: DeserializeOwned>(&self, token: &str) -> JwtAuthResult<UserClaims> {
        self.coder.decode(token)
    }
}