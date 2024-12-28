use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims<UserPartClaims>
    where UserPartClaims: Serialize {
    pub user_claims: UserPartClaims,
    pub exp: i64
}