use secrecy::{ExposeSecret, SecretString};
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::models::User;
use crate::api::repositories::{
    fetch_all_users, fetch_credentials_by_user_id, update_user_password,
};
use crate::api::services::{compute_password_hash, verify_password_hash};
use crate::api::utils::{Metadata, QueryParams};
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
