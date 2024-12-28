use crate::auth::jwt::AuthError;
use axum::http::{StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use dill::InjectionError;
use serde_json::json;
use szfit_domain::repositories::error::RepositoryError;
use szfit_domain::services::error::ServiceError;

pub type WebResult<T> = Result<T, WebError>;
pub type JsonResult<T> = WebResult<(StatusCode, Json<T>)>;

#[derive(thiserror::Error, derive_more::Display, Debug, Default)]
pub enum WebError {
    RepoError(#[from] RepositoryError),
    ServiceError(#[from] ServiceError),
    AuthError(#[from] AuthError),
    #[default]
    Other,
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            WebError::RepoError(_) => { (StatusCode::INTERNAL_SERVER_ERROR, "Internal Service Error") }
            WebError::ServiceError(_) => { (StatusCode::INTERNAL_SERVER_ERROR, "Internal Service Error") }
            WebError::Other => { (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error") }
            WebError::AuthError(_) => { (StatusCode::UNAUTHORIZED, "Need Auth Role") }
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