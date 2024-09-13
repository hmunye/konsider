use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::server::AppState;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::{Error, Result, UserRole};

// ---------------------------------------------------------------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "fetching user role", skip(user_id, db_pool))]
pub async fn get_user_role(user_id: Uuid, db_pool: &PgPool) -> Result<UserRole> {
    let row = sqlx::query!(
        r#"
        SELECT role AS "role: UserRole"
        FROM users
        WHERE id = $1
        "#,
        user_id,
    )
    .fetch_one(db_pool)
    .await
    .map_err(|err| {
        Error::UnexpectedError(
            std::sync::Arc::new(err),
            "Failed to fetch user role from database".into(),
        )
    })?;

    Ok(row.role as UserRole)
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "validating user credentials", skip(state, payload))]
pub async fn validate_credentials(state: &AppState, payload: Credentials) -> Result<Uuid> {
    // When attempting to validate credentails, passing an incorrect email and password takes
    // and order of magnitude less of time then with a correct email and incorrect password
    //
    // Fallback user_id and expected_password_hash are used so that the same operations happen during
    // each scenario and no notable time difference can be observed
    //
    // Ex. Timing attacks (side-channel attack)
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_password_hash)) =
        get_credentials(state, &payload.email).await?
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
    .map_err(|err| {
        Error::UnexpectedError(
            std::sync::Arc::new(err),
            "Failed to spawn blocking thread for password hashing".into(),
        )
    })??;

    user_id.ok_or_else(|| {
        Error::EmailNotFoundError("Failed to find email associated with user".into())
    })
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "fetching stored credentials", skip(state, email))]
async fn get_credentials(
    state: &AppState,
    email: &str,
) -> Result<Option<(uuid::Uuid, Secret<String>)>> {
    let row = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM users
        WHERE email = $1
        "#,
        email,
    )
    .fetch_optional(&state.db_pool)
    .await
    .map_err(|err| {
        Error::UnexpectedError(
            std::sync::Arc::new(err),
            "Failed to fetch user credentials from database".into(),
        )
    })?
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
) -> Result<()> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .map_err(|err| {
            Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to parse password hash".into(),
            )
        })?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .map_err(|err| Error::InvalidPasswordError(err.to_string()))?;

    Ok(())
}
