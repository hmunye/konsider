use axum::extract::{self, State};
use axum::http::StatusCode;

use crate::server::AppState;
use crate::{Error, Result, User, UserRole};

#[tracing::instrument(
    name = "creating new user", 
    // Any values in 'skip' won't be included in logs
    skip(state, payload),
    fields(
        user_email = %payload.email,
    )
)]
pub async fn api_create_user(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<User>,
) -> Result<StatusCode> {
    // Validate payload from request
    payload.parse()?;

    // TODO: Hash and salt password
    let password_hash = &payload.password;

    insert_user(&state, &payload, password_hash.to_string()).await?;

    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "inserting user in db", skip(state, payload, password_hash))]
async fn insert_user(state: &AppState, payload: &User, password_hash: String) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO "user" (name, email, password_hash, role)
        VALUES ($1, $2, $3, $4)
        "#,
        payload.name,
        payload.email,
        password_hash,
        payload.role.clone() as UserRole,
    )
    .execute(&state.db_pool)
    .await
    .map_err(|err| Error::InsertUserError(err.to_string()))?;

    Ok(())
}
