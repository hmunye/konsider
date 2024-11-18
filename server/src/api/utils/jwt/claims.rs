use serde::{Deserialize, Serialize};

use crate::api::models::UserRole;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub role: UserRole,
    pub iat: usize,
    pub exp: usize,
    pub jti: uuid::Uuid,
}
