use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use secrecy::SecretString;
use serde::Deserialize;
use serde_json::json;

use crate::api::models::UserRole;
use crate::api::services::{change_user_password, get_all_users, revoke_user_token};
use crate::api::utils::{Json, QueryExtractor, Token};
use crate::server::ServerState;
use crate::{Error, Result};

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

#[tracing::instrument(
    name = "get all users", 
    // Any values in 'skip' won't be included in logs
    skip(token, query_params, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_get_all_users(
    Token(token): Token,
    QueryExtractor(query_params): QueryExtractor,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    // Only allow `ADMIN` users to access this endpoint
    match token.role {
        UserRole::ADMIN => (),
        _ => return Err(Error::AuthInvalidRoleError)?,
    }

    let (users, metadata) = get_all_users(query_params.0, &state.db_pool).await?;

    let response_body = json!({
        "metadata": if metadata.total_records == 0 {
            json!({})
        } else {
            json!(metadata)
        },
        "users": users
    });

    Ok((StatusCode::OK, Json(response_body)))
}
