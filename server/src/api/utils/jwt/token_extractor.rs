use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::header;
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};

use crate::api::utils::jwt::{decode_jwt, Claims};
use crate::server::ServerState;
use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    ServerState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    // Checks for JWT in request, validate JWT, and return the claims of JWT
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::COOKIE)
            .and_then(|value| value.to_str().ok())
            .and_then(|str| str.split("=").nth(1));

        match token {
            Some(token) => {
                let state = ServerState::from_ref(state);

                let decoded_token = decode_jwt(token, &state.jwt_secret)?;

                Ok(Token(decoded_token.claims))
            }

            None => Err(Error::AuthMissingTokenError),
        }
    }
}
