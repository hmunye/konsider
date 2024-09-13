use axum::extract::{self, State};
use axum::http::StatusCode;

use crate::model::TypedSession;
use crate::server::AppState;
use crate::web::auth::{validate_credentials, Credentials};
use crate::Result;

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "logging in user", 
    // Any values in 'skip' won't be included in logs
    skip(state, session, payload),
    fields(
        user_email = %payload.email
    )
)]
pub async fn api_login(
    State(state): State<AppState>,
    session: TypedSession,
    extract::Json(payload): extract::Json<Credentials>,
) -> Result<StatusCode> {
    let user_id = validate_credentials(&state, payload).await?;

    // Rotating session id prevents session fixation attacks
    session.cycle().await?;

    // Create session with user id
    session.insert_user_id(user_id).await?;

    //    let user_role = get_user_role(user_id, &state.db_pool).await?;
    //
    //    Ok(Response::builder()
    //        .status(StatusCode::OK)
    //        .header(header::CONTENT_TYPE, "application/json")
    //        .body(json!({"role": user_role.to_string()}).to_string())
    //        .unwrap_or_default())

    Ok(StatusCode::OK)
}
