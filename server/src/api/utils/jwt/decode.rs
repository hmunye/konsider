use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use secrecy::{ExposeSecret, SecretString};

use crate::api::utils::jwt::Claims;
use crate::{Error, Result};

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
