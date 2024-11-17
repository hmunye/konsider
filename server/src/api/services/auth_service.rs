use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::repositories::{
    fetch_credentials_by_email, fetch_credentials_by_user_id, update_user_password,
};
use crate::api::UserRole;
use crate::log::spawn_blocking_with_tracing;
use crate::{Error, Result};

#[tracing::instrument(name = "validating user credentials", skip(email, password, db_pool))]
pub async fn validate_credentials<'a>(
    email: &'a str,
    password: SecretString,
    db_pool: &PgPool,
) -> Result<(Uuid, UserRole)> {
    // When attempting to validate credentials, passing an incorrect email and password takes
    // and order of magnitude less of time then with a correct email and incorrect password
    //
    // Fallback user_id and expected_password_hash are used so that the same operations happen during
    // each scenario and no notable time difference can be observed
    //
    // Ex. Timing attacks (side-channel attack)
    let mut user_id = None;
    let mut expected_password_hash = SecretString::new(
        "$argon2id$v=19$m=19456,t=2,p=1$sCz8l1doj9fIezPbGeudnA$OOFnWka6++Q9r7FEy1d2WhmW7FXwR9uVkQAB/baIJW8".into(),
    );
    let mut user_role = None;

    if let Some((stored_user_id, stored_password_hash, stored_user_role)) =
        fetch_credentials_by_email(email, db_pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
        user_role = Some(stored_user_role)
    }

    // Using tokio to spawn a thread pool for blocking operations
    // Ex. CPU intensive tasks like password hashing
    spawn_blocking_with_tracing(move || verify_password_hash(expected_password_hash, password))
        .await??;

    let user_id = user_id.ok_or_else(|| Error::AuthEmailNotFoundError)?;
    let user_role = user_role.ok_or_else(|| Error::AuthEmailNotFoundError)?;

    Ok((user_id, user_role))
}

#[tracing::instrument(
    name = "changing user password",
    skip(user_id, current_password, new_password, db_pool)
)]
// TODO: Update to check if passwords provided are the same first
pub async fn change_user_password(
    user_id: Uuid,
    current_password: SecretString,
    new_password: SecretString,
    db_pool: &PgPool,
) -> Result<()> {
    let mut expected_password_hash = SecretString::new(
        "$argon2id$v=19$m=19456,t=2,p=1$sCz8l1doj9fIezPbGeudnA$OOFnWka6++Q9r7FEy1d2WhmW7FXwR9uVkQAB/baIJW8".into(),
    );

    if let Some((stored_password_hash, _)) = fetch_credentials_by_user_id(user_id, db_pool).await? {
        expected_password_hash = stored_password_hash;
    }

    // Using tokio to spawn a thread pool for blocking operations
    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, current_password)
    })
    .await??;

    let password_hash = compute_password_hash(new_password)?;

    update_user_password(user_id, password_hash, db_pool).await
}

#[tracing::instrument(
    name = "verifying password hash",
    skip(expected_password_hash, password_candidate)
)]
fn verify_password_hash(
    expected_password_hash: SecretString,
    password_candidate: SecretString,
) -> Result<()> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .map_err(|_| Error::AuthInvalidPasswordError)?;

    Ok(())
}

#[tracing::instrument(name = "computing password hash", skip(password))]
fn compute_password_hash(password: SecretString) -> Result<SecretString> {
    let salt = SaltString::generate(&mut rand::thread_rng());

    let password_hash = Argon2::default()
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?
        .to_string();

    Ok(SecretString::new(password_hash.into()))
}
