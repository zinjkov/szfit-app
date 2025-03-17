use async_trait::async_trait;
use axum::{
    extract::{FromRef, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
    RequestExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use dill::Catalog;
use std::convert::Infallible;
use szfit_domain::services::jwt_service::IJwtAuthenticator;

pub async fn auth_middleware(
    CatalogExtractor(cat): CatalogExtractor, mut req: Request, next: Next,
) -> Result<Response, StatusCode> {
    let TypedHeader(Authorization(bearer)) = req
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let authenticator = cat
        .get_one::<dyn IJwtAuthenticator>()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let auth_claims = authenticator
        .authenticate(bearer.token())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(auth_claims);
    Ok(next.run(req).await)
}

pub struct CatalogExtractor(pub Catalog);

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for CatalogExtractor
where
    Catalog: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts, state: &S,
    ) -> Result<Self, Self::Rejection> {
        let catalog = Catalog::from_ref(state.clone());
        Ok(Self(catalog))
    }
}
