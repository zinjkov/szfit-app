use crate::jwt_auth::error::JwtAuthError;
use crate::repositories::error::RepositoryError;

#[derive(thiserror::Error, derive_more::Display, Debug, Default)]
pub enum ServiceError {
    RepositoryError(#[from] RepositoryError),
    JwtCodingError(#[from] JwtAuthError),
    RefreshTokenIncorrect,
    #[default]
    Other
}

pub type ServiceResult<T> = Result<T, ServiceError>;