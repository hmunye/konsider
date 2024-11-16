use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

use crate::api::utils::jwt::Claims;
use crate::api::UserRole;
use crate::{Error, Result};

// Define the validity duration in minutes. Approximately 1 month
const TOKEN_VALIDITY_DURATION: i64 = 43_800;

// Returns the header and claims of JWT
pub fn decode_jwt(token: &str, secret: &SecretString) -> Result<TokenData<Claims>> {
    let mut validation = Validation::new(Algorithm::HS256);

    validation.set_required_spec_claims(&["sub", "iat", "exp"]);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.expose_secret().as_bytes()),
        &validation,
    )
    .map_err(|_| Error::AuthInvalidTokenError)?;

    Ok(token_data)
}

pub fn generate_jwt(user_id: &Uuid, user_role: UserRole, secret: &SecretString) -> Result<String> {
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
            jti: Uuid::new_v4(),
        },
        &EncodingKey::from_secret(secret.expose_secret().as_bytes()),
    )?;

    Ok(token)
}
