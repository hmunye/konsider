use axum::body::Body;
use axum::extract::State;
use axum::http::header::{CONTENT_TYPE, SET_COOKIE};
use axum::http::StatusCode;
use axum::response::Response;
use secrecy::SecretString;
use serde::Deserialize;
use serde_json::json;

use crate::api::services::{revoke_user_token, save_user_token, validate_credentials};
use crate::api::utils::{generate_jwt, Cookie, Json, SameSite, Token};
use crate::server::ServerState;
use crate::Result;

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
) -> Result<Response<Body>> {
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
    // cookie.set_secure();
    cookie.set_same_site(SameSite::Strict);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .header(SET_COOKIE, cookie.build())
        .body(axum::body::Body::from(
            json!({
                "role": user_role
            })
            .to_string(),
        ))
        .unwrap())
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
) -> Result<StatusCode> {
    tracing::Span::current().record("request_initiator", tracing::field::display(&token.sub));

    revoke_user_token(token.jti, &state.db_pool).await?;

    state.token_cache.remove_token(token.jti, token.sub).await;

    Ok(StatusCode::NO_CONTENT)
}
