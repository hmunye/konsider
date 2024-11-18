use axum::extract::State;
use axum::http::StatusCode;
use secrecy::SecretString;
use serde::Deserialize;

use crate::api::services::{change_user_password, revoke_user_token};
use crate::api::utils::{Json, Token};
use crate::server::ServerState;
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct ChangePasswordPayload {
    current_password: SecretString,
    new_password: SecretString,
}

#[tracing::instrument(
    name = "user change password", 
    skip(token, state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_change_password(
    Token(token): Token,
    State(state): State<ServerState>,
    Json(payload): Json<ChangePasswordPayload>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    change_user_password(
        token.sub,
        payload.current_password,
        payload.new_password,
        &state.db_pool,
    )
    .await?;

    // Invalidate user token after successful password update
    revoke_user_token(token.jti, &state.db_pool).await?;
    state.token_cache.remove_token(token.jti, token.sub).await;

    Ok(StatusCode::NO_CONTENT)
}
