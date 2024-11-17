use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse};
use secrecy::SecretString;
use serde::Deserialize;

use crate::api::services::{revoke_user_token, save_user_token, validate_credentials};
use crate::api::{generate_jwt, Cookie, Json, SameSite, Token};
use crate::server::ServerState;
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: SecretString,
}

#[tracing::instrument(
    name = "user login", 
    skip(state, payload),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_login(
    State(state): State<ServerState>,
    Json(payload): Json<Credentials>,
) -> Result<impl IntoResponse> {
    let (user_id, user_role) = validate_credentials(&state.db_pool, payload).await?;

    tracing::Span::current().record("request_initiator", tracing::field::display(&user_id));

    let (token, jti) = generate_jwt(&user_id, user_role, &state.jwt_secret)?;

    save_user_token(jti, &user_id, &state.db_pool).await?;
    state.token_cache.insert_token(jti, user_id).await;

    let mut cookie = Cookie::new(token);

    cookie.set_domain("localhost");
    cookie.set_path("/");
    cookie.set_http_only();
    // cookie.set_secure();
    cookie.set_same_site(SameSite::Strict);

    let headers = AppendHeaders([(SET_COOKIE, cookie.build_header())]);

    Ok((StatusCode::NO_CONTENT, headers))
}

#[tracing::instrument(
    name = "user logout", 
    skip(token, state),
    fields(
        request_initiator = tracing::field::Empty,
    )
)]
pub async fn api_logout(
    Token(token): Token,
    State(state): State<ServerState>,
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    revoke_user_token(token.jti, &state.db_pool).await?;

    state.token_cache.remove_token(token.jti, token.sub).await;

    Ok(StatusCode::NO_CONTENT)
}
