use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::User;
use crate::api::repositories::{fetch_credentials_by_user_id, update_user_password};
use crate::api::services::{compute_password_hash, verify_password_hash};
use crate::log::spawn_blocking_with_tracing;
use crate::{Error, Result};

#[tracing::instrument(
    name = "changing user password",
    skip(user_id, current_password, new_password, db_pool)
)]
pub async fn change_user_password(
    user_id: Uuid,
    current_password: SecretString,
    new_password: SecretString,
    db_pool: &PgPool,
) -> Result<()> {
    if new_password.expose_secret() == current_password.expose_secret() {
        return Err(Error::ValidationError(
            "change password payload: passwords provided should not match".into(),
        ));
    }

    if !User::validate_password(&new_password.expose_secret().to_string()) {
        return Err(Error::ValidationError(
            "change password payload: invaild password provided by user".into(),
        ));
    }

    let mut expected_password_hash = SecretString::new(
        "$argon2id$v=19$m=19456,t=2,p=1$sCz8l1doj9fIezPbGeudnA$OOFnWka6++Q9r7FEy1d2WhmW7FXwR9uVkQAB/baIJW8".into(),
    );

    if let Some((stored_password_hash, _)) = fetch_credentials_by_user_id(user_id, db_pool).await? {
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, current_password)
    })
    .await??;

    let password_hash = compute_password_hash(new_password)?;

    update_user_password(user_id, password_hash, db_pool).await
}
