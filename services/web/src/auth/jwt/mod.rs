use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use dill::Catalog;
use serde_json::json;
use std::ops::Deref;
use szfit_domain::services::jwt_service::{AuthClaims, IJwtAuthenticator};

pub struct ExctractAuthClaims(pub AuthClaims);

impl Deref for ExctractAuthClaims {
    type Target = AuthClaims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(thiserror::Error, derive_more::Display, Debug)]
pub enum AuthError {
    WrongCredentials,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong credentials")
            }
            AuthError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid Token")
            }
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for ExctractAuthClaims
where
    Catalog: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let catalog = Catalog::from_ref(state);
        let authenticator =
            catalog.get_one::<dyn IJwtAuthenticator>().unwrap();
        let auth_claims =
            authenticator.authenticate(bearer.token()).unwrap();
        Ok(ExctractAuthClaims(auth_claims))
    }
}
