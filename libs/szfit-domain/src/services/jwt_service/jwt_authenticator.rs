use crate::services::error::ServiceResult;
use crate::services::jwt_service::auth_claims::AuthClaims;

pub trait IJwtAuthenticator: Send + Sync {
    fn authenticate(&self, token: &str) -> ServiceResult<AuthClaims>;
}
