use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

use crate::api::utils::jwt::Claims;
use crate::api::UserRole;
use crate::{Error, Result};

// 24 hours = 24 * 60 minutes = 1440 minutes
const TOKEN_VALIDITY_DURATION: i64 = 1440;

// Returns JWT and unique identifier
pub fn generate_jwt(
    user_id: &Uuid,
    user_role: UserRole,
    secret: &SecretString,
) -> Result<(String, Uuid)> {
    let jti = Uuid::new_v4();

    let header = Header {
        alg: Algorithm::HS256,
        ..Default::default()
    };

    let token = encode(
        &header,
        &Claims {
            sub: *user_id,
            role: user_role,
            iat: (chrono::Utc::now()).timestamp() as usize,
            exp: (chrono::Utc::now() + chrono::Duration::minutes(TOKEN_VALIDITY_DURATION))
                .timestamp() as usize,
            jti,
        },
        &EncodingKey::from_secret(secret.expose_secret().as_bytes()),
    )
    .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?;

    Ok((token, jti))
}
