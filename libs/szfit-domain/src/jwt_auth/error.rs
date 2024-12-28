
#[derive(thiserror::Error, derive_more::Display, Debug, Default)]
pub enum JwtAuthError {
    TokenEncodeError(#[from] jsonwebtoken::errors::Error),
    #[default]
    Other
}

pub type JwtAuthResult<T> = Result<T, JwtAuthError>;