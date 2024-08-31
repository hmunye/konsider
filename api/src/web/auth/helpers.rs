use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use uuid::Uuid;

use crate::server::AppState;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::ServerError;

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "validating credentials", skip(state, payload))]
pub async fn validate_credentials(
    state: &AppState,
    payload: Credentials,
) -> Result<Uuid, ServerError> {
    // When attempting to validate credentails, passing an incorrect email and password takes
    // and order of magnitude less of time then with a correct email and incorrect password
    //
    // Fallback user_id and expected_password_hash are used so that the same operations happen during
    // each scenario and no notable time difference can be observed
    //
    // Timing attacks (side-channel)
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) = get_credentials(state, &payload.email)
        .await
        .map_err(|err| ServerError::DatabaseError(err.to_string()))?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    // Using tokio to spawn a thread pool for blocking operations
    // Ex. CPU intensive tasks like password hashing
    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, payload.password)
    })
    .await
    .map_err(|err| ServerError::LoginError(err.to_string()))??;

    user_id.ok_or_else(|| ServerError::FetchUserError("email not found".to_string()))
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "getting stored credentials", skip(state, email))]
async fn get_credentials(
    state: &AppState,
    email: &str,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, ServerError> {
    let row = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM "user"
        WHERE email = $1
        "#,
        email,
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|err| ServerError::DatabaseError(err.to_string()))?
    .map(|row| (row.id, Secret::new(row.password_hash)));

    Ok(row)
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "verifying password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), ServerError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .map_err(|err| ServerError::UnexpectedError(err.to_string()))?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .map_err(|err| ServerError::LoginError(err.to_string()))
}
