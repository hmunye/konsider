use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use secrecy::SecretString;
use serde::Deserialize;
use serde_json::json;

use crate::api::models::{User, UserRole};
use crate::api::services::{
    change_user_password, create_user, get_all_users, get_user, remove_user, revoke_user_token,
    update_user_details,
};
use crate::api::utils::{Json, Path, QueryExtractor, Token};
use crate::api::UserDTO;
use crate::server::ServerState;
use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct ChangePasswordPayload {
    current_password: SecretString,
    new_password: SecretString,
}

#[tracing::instrument(
    name = "user change password", 
    // Any values in 'skip' won't be included in logs
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

#[tracing::instrument(
    name = "get user", 
    // Any values in 'skip' won't be included in logs
    skip(token, user_id, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_get_user(
    Token(token): Token,
    Path(user_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    // Only allow `ADMIN` users to access this endpoint
    match token.role {
        UserRole::ADMIN => (),
        _ => return Err(Error::AuthInvalidRoleError)?,
    }

    let user = get_user(user_id, &state.db_pool).await?;

    let response_body = json!({
        "user": UserDTO::from(&user)
    });

    Ok((StatusCode::OK, Json(response_body)))
}

#[tracing::instrument(
    name = "create user", 
    // Any values in 'skip' won't be included in logs
    skip(token, state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_create_user(
    Token(token): Token,
    State(state): State<ServerState>,
    Json(payload): Json<User>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    // Only allow `ADMIN` users to access this endpoint
    match token.role {
        UserRole::ADMIN => (),
        _ => return Err(Error::AuthInvalidRoleError)?,
    }

    payload.parse()?;

    create_user(&payload, &state.db_pool).await?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(
    name = "delete user", 
    // Any values in 'skip' won't be included in logs
    skip(token, user_id, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_delete_user(
    Token(token): Token,
    Path(user_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    // Only allow `ADMIN` users to access this endpoint
    match token.role {
        UserRole::ADMIN => (),
        _ => return Err(Error::AuthInvalidRoleError)?,
    }

    remove_user(user_id, &state.db_pool).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPayload {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<UserRole>,
}

#[tracing::instrument(
    name = "update user details", 
    // Any values in 'skip' won't be included in logs
    skip(token, user_id, state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_update_user(
    Token(token): Token,
    Path(user_id): Path<uuid::Uuid>,
    State(state): State<ServerState>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    // Only allow `ADMIN` users to access this endpoint
    match token.role {
        UserRole::ADMIN => (),
        _ => return Err(Error::AuthInvalidRoleError)?,
    }

    update_user_details(payload, user_id, &state.db_pool).await?;

    Ok(StatusCode::NO_CONTENT)
}
