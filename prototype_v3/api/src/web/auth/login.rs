use axum::extract::{self, State};
use axum::http::StatusCode;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use tower_sessions::Session;

use crate::server::AppState;
use crate::web::auth::AUTH_TOKEN;
use crate::web::auth::{validate_credentials, Credentials};
use crate::{Error, Result};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "user login attempt", 
    // Any values in 'skip' won't be included in logs
    skip(state, payload),
    fields(
        user_email = tracing::field::Empty
    )
)]
pub async fn api_login(
    State(state): State<AppState>,
    session: Session,
    cookies: Cookies,
    extract::Json(payload): extract::Json<Credentials>,
) -> Result<StatusCode> {
    tracing::Span::current().record("user_email", tracing::field::display(&payload.email));

    match validate_credentials(&state, payload).await {
        Ok(user_id) => {
            // Rotating session token prevents session fixation attacks
            session
                .cycle_id()
                .await
                .map_err(|err| Error::UnexpectedError(err.to_string()))?;

            // Create session with user id
            session
                .insert("user_id", user_id)
                .await
                .map_err(|err| Error::UnexpectedError(err.to_string()))?
        }
        Err(err) => return Err(Error::FetchUserError(err.to_string())),
    };

    let mut cookie = Cookie::new(AUTH_TOKEN, session.id().is_none().to_string());

    // TODO: Change these options later on
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_domain("localhost");
    cookie.set_same_site(SameSite::None);
    cookie.set_secure(false);

    cookies.add(cookie);

    Ok(StatusCode::OK)
}
