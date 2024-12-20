use secrecy::{ExposeSecret, SecretString};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::controllers::UpdateUserPayload;
use crate::api::models::User;
use crate::api::repositories::{
    delete_user, fetch_all_users, fetch_credentials_by_user_id, fetch_user_by_id, insert_user,
    update_user, update_user_password,
};
use crate::api::services::{compute_password_hash, verify_password_hash};
use crate::api::utils::{Metadata, QueryParams};
use crate::api::UserDTO;
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

    if !User::validate_password(new_password.expose_secret()) {
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

    let password_hash = compute_password_hash(&new_password)?;

    update_user_password(user_id, password_hash, db_pool).await
}

#[tracing::instrument(name = "getting all users", skip(query_params, db_pool))]
pub async fn get_all_users(
    query_params: QueryParams,
    db_pool: &PgPool,
) -> Result<(Vec<Value>, Metadata)> {
    let sort_safe_list = [
        "name".to_string(),
        "email".to_string(),
        "role".to_string(),
        "-name".to_string(),
        "-email".to_string(),
        "-role".to_string(),
    ];

    let filter_safe_list = ["name".to_string(), "email".to_string(), "role".to_string()];

    query_params.parse(&sort_safe_list, &filter_safe_list)?;

    let page = query_params.page.unwrap_or(1);
    let per_page = query_params.per_page.unwrap_or(10);

    let (sort_column, sort_direction) = match query_params
        .sort
        .unwrap_or("id".to_string())
        .strip_prefix("-")
    {
        Some(sort_column) => (sort_column.to_string(), "DESC".to_string()),
        None => ("id".to_string(), "ASC".to_string()),
    };

    let mut filter_field = None;
    let mut filter_value = None;

    if let Some(filter_str) = query_params.filter {
        let parts: Vec<&str> = filter_str.split(':').collect();

        if parts.len() == 2 {
            let field = parts[0].to_string();
            let value = parts[1].to_string();

            filter_field = Some(field);
            filter_value = Some(value);
        }
    }

    let (users, metadata) = fetch_all_users(
        sort_column,
        sort_direction,
        page,
        per_page,
        filter_field,
        filter_value,
        db_pool,
    )
    .await?;

    let wrapped_users: Vec<Value> = users
        .into_iter()
        .map(|user| {
            json!({
                "user": user
            })
        })
        .collect();

    Ok((wrapped_users, metadata))
}

#[tracing::instrument(name = "getting user", skip(user_id, db_pool))]
pub async fn get_user_by_id(user_id: Uuid, db_pool: &PgPool) -> Result<UserDTO> {
    let user = fetch_user_by_id(user_id, db_pool).await?;

    Ok(UserDTO::from(&user))
}

#[tracing::instrument(name = "creating user", skip(payload, db_pool))]
pub async fn create_user(payload: &User, db_pool: &PgPool) -> Result<Uuid> {
    let password_hash = compute_password_hash(&payload.password)?;

    insert_user(payload, password_hash, db_pool).await
}

#[tracing::instrument(name = "removing user", skip(user_id, db_pool))]
pub async fn remove_user(user_id: Uuid, db_pool: &PgPool) -> Result<()> {
    delete_user(user_id, db_pool).await
}

#[tracing::instrument(name = "updating user", skip(payload, user_id, db_pool))]
pub async fn update_user_details(
    payload: UpdateUserPayload,
    user_id: Uuid,
    db_pool: &PgPool,
) -> Result<()> {
    // Fetch user from database if a record exists
    let mut user = fetch_user_by_id(user_id, db_pool).await?;

    let mut fields_updated = false;

    // Apply any updates to the `User` entity locally
    if let Some(name) = &payload.name {
        user.name = name.clone();
        fields_updated = true;
    }

    if let Some(email) = &payload.email {
        user.email = email.clone();
        fields_updated = true;
    }

    if let Some(role) = &payload.role {
        user.role = role.clone();
        fields_updated = true;
    }

    // Return an error if no fields were updated
    if !fields_updated {
        return Err(Error::NoUpdatesProvidedError);
    }

    user.parse_without_password()?;

    update_user(user, user_id, db_pool).await
}
