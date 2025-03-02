use crate::entity::{Id, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthClaims {
    pub user_id: Id,
}

impl From<&User> for AuthClaims {
    fn from(value: &User) -> Self {
        Self {
            user_id: value.id,
        }
    }
}
