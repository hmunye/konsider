use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::extract::{self, State};
use axum::http::StatusCode;
use secrecy::ExposeSecret;

use crate::server::AppState;
use crate::{Error, Result, User, UserRole};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "creating new user", 
    // Any values in 'skip' won't be included in logs
    skip(state, payload),
    fields(
        user_email = tracing::field::Empty,
    )
)]
pub async fn api_create_user(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<User>,
) -> Result<StatusCode> {
    tracing::Span::current().record("user_email", tracing::field::display(&payload.email));

    // Validate request payload
    payload.parse()?;

    let password_hash = hash_and_salt_password(payload.password.expose_secret())?;

    insert_user(&state, &payload, password_hash).await?;

    Ok(StatusCode::OK)
}
// ---------------------------------------------------------------------------------------------------------------
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
// ---------------------------------------------------------------------------------------------------------------
fn hash_and_salt_password(password: &String) -> Result<String> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| Error::UnexpectedError(err.to_string()))?
        .to_string();

    Ok(password_hash)
}
