use std::ops::Deref;

mod jwt_service;
pub mod error;
mod jwt_coder;
pub mod claims;
pub mod jwt_generator;
pub mod jwt_validator;


#[derive(Clone)]
pub struct JwtSecret(pub String);

impl Deref for JwtSecret {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub mod prelude {
    pub use crate::jwt_auth::jwt_generator::*;
    pub use crate::jwt_auth::jwt_validator::*;
}