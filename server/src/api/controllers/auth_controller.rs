use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse};
use secrecy::SecretString;
use serde::Deserialize;

use crate::api::services::validate_credentials;
use crate::api::{generate_jwt, Cookie, Json, SameSite};
use crate::server::ServerState;
use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: SecretString,
}

#[tracing::instrument(
    name = "user login", 
    skip(payload, state),
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

    let token = generate_jwt(&user_id, user_role, &state.jwt_secret)?;

    let mut cookie = Cookie::new(token);

    cookie.set_domain("localhost");
    cookie.set_path("/");
    cookie.set_http_only();
    // cookie.set_secure();
    cookie.set_same_site(SameSite::Strict);

    let headers = AppendHeaders([(SET_COOKIE, cookie.build_header())]);

    Ok((StatusCode::OK, headers))
}
