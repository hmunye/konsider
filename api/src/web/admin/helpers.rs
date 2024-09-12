use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

use crate::server::AppState;
use crate::{Error, Result, User, UserRole};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "inserting user in db", skip(state, payload, password_hash))]
pub async fn insert_user(state: &AppState, payload: &User, password_hash: String) -> Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO users (name, email, password_hash, role)
        VALUES ($1, $2, $3, $4)
        "#,
        payload.name,
        payload.email,
        password_hash,
        payload.role.clone() as UserRole,
    )
    .execute(&state.db_pool)
    .await
    .map_err(Error::from)?;

    Ok(())
}
// ---------------------------------------------------------------------------------------------------------------
pub fn compute_password_hash(password: &String) -> Result<String> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| {
            Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to compute password hash".into(),
            )
        })?
        .to_string();

    Ok(password_hash)
}
