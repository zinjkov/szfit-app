use crate::auth::jwt::AuthError;
use axum::{http::StatusCode, response::IntoResponse, Json};
use dill::InjectionError;
use serde_json::json;
use szfit_domain::{
    repositories::error::RepositoryError, services::error::ServiceError,
};

pub type WebResult<T> = Result<T, WebError>;
pub type JsonResult<T> = WebResult<(StatusCode, Json<T>)>;

#[derive(Debug, thiserror::Error, Default)]
pub enum WebError {
    #[error("Repository error: {0}")]
    RepoError(#[from] RepositoryError),
    #[error("Service error: {0}")]
    ServiceError(#[from] ServiceError),
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),
    #[error("Other error")]
    #[default]
    Other,
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            WebError::RepoError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Service Error")
            }
            WebError::ServiceError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Service Error")
            }
            WebError::Other => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            WebError::AuthError(_) => {
                (StatusCode::UNAUTHORIZED, "Need Auth Role")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));
        log::error!("{:#?}", self);
        (status, body).into_response()
    }
}

impl From<InjectionError> for WebError {
    fn from(_: InjectionError) -> Self {
        Self::Other
    }
}

pub trait IntoJsonResult<T> {
    fn into_json(self) -> JsonResult<T>;
}

impl<T> IntoJsonResult<T> for WebResult<T> {
    fn into_json(self) -> JsonResult<T> {
        self.map(|t| (StatusCode::OK, Json(t)))
    }
}
