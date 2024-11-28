use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse};
use secrecy::SecretString;
use serde::Deserialize;
use serde_json::json;

use crate::api::models::UserRole;
use crate::api::services::{
    get_user_by_id, revoke_user_token, save_user_token, validate_credentials,
};
use crate::api::utils::{generate_jwt, Cookie, Json, Path, SameSite, Token};
use crate::server::ServerState;
use crate::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct CredentialsPayload {
    email: String,
    password: SecretString,
}

#[tracing::instrument(
    name = "user login", 
    // Any values in 'skip' won't be included in logs
    skip(state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_login(
    State(state): State<ServerState>,
    Json(payload): Json<CredentialsPayload>,
) -> Result<impl IntoResponse> {
    let (user_id, user_role) =
        validate_credentials(&payload.email, payload.password, &state.db_pool).await?;

    tracing::Span::current().record("request_initiator", tracing::field::display(&user_id));

    let (token, jti) = generate_jwt(&user_id, user_role.clone(), &state.jwt_secret)?;

    save_user_token(jti, &user_id, &state.db_pool).await?;
    state.token_cache.insert_token(jti, user_id).await;

    let mut cookie = Cookie::new(token);

    cookie.set_domain("localhost");
    cookie.set_path("/");
    cookie.set_http_only();

    if state.environment.as_str() == "production" {
        cookie.set_secure();
        // Needs to be `SameSite::None` for cross-origin requests
        cookie.set_same_site(SameSite::None);
    } else {
        cookie.set_same_site(SameSite::Lax);
    }

    let headers = AppendHeaders([(SET_COOKIE, cookie.build())]);

    Ok((StatusCode::NO_CONTENT, headers))
}

#[tracing::instrument(
    name = "user logout", 
    // Any values in 'skip' won't be included in logs
    skip(token, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_logout(
    Token(token): Token,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    let _ = revoke_user_token(token.sub, &state.db_pool).await?;

    state.token_cache.remove_token(token.jti, token.sub).await;

    let cookie = Cookie::clear("localhost", "/");

    let headers = AppendHeaders([(SET_COOKIE, cookie.build())]);

    Ok((StatusCode::NO_CONTENT, headers))
}

#[tracing::instrument(
    name = "user token check", 
    skip(token, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
// `Token` will check if a vaild JWT was provided
pub async fn api_check_token(
    Token(token): Token,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    let user_dto = get_user_by_id(token.sub, &state.db_pool).await?;

    let response_body = json!({
        "user": user_dto
    });

    Ok((StatusCode::OK, Json(response_body)))
}

#[tracing::instrument(
    name = "revoke user token", 
    skip(token, user_id, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_revoke_user_token(
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

    let jti = revoke_user_token(user_id, &state.db_pool).await?;

    state.token_cache.remove_token(jti, user_id).await;

    Ok(StatusCode::NO_CONTENT)
}
