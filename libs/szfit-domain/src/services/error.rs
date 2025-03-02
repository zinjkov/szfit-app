use crate::jwt_auth::error::JwtAuthError;
use crate::repositories::error::RepositoryError;

#[derive(thiserror::Error, Debug, Default)]
pub enum ServiceError {
    #[error("An error occurred while accessing the repository: {0}")]
    RepositoryError(#[from] RepositoryError),
    #[error("An error occurred while encoding/decoding JWT: {0}")]
    JwtCodingError(#[from] JwtAuthError),
    #[error("The provided refresh token is incorrect")]
    RefreshTokenIncorrect,
    #[error("An unknown error occurred")]
    #[default]
    Other,
}

pub type ServiceResult<T> = Result<T, ServiceError>;
