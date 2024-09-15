use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

use crate::server::AppState;
use crate::{Error, Result, User, UserRole};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "fetching user role", skip(user_id, db_pool))]
pub async fn get_user_role(user_id: Uuid, db_pool: &PgPool) -> Result<UserRole> {
    match sqlx::query!(
        r#"
        SELECT role AS "role: UserRole"
        FROM users
        WHERE id = $1
        "#,
        user_id,
    )
    .fetch_one(db_pool)
    .await
    {
        Ok(row) => Ok(row),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(Error::NotFoundError),
            _ => Err(Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to fetch user role from database".into(),
            )),
        },
    }
    .map(|row| row.role as UserRole)
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "fetching user by id", skip(state, user_id))]
pub async fn fetch_user_by_id(state: &AppState, user_id: &Uuid) -> Result<User> {
    match sqlx::query!(
        r#"
        SELECT name, email, password_hash, role AS "role: UserRole", version 
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(row) => Ok(row),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(Error::NotFoundError),
            _ => Err(Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to fetch user by id from database".into(),
            )),
        },
    }
    .map(|row| User {
        name: row.name,
        email: row.email,
        password: Secret::new(row.password_hash),
        role: row.role,
        version: row.version,
    })
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(
    name = "inserting user into database",
    skip(state, payload, password_hash)
)]
pub async fn insert_user(state: &AppState, payload: &User, password_hash: String) -> Result<()> {
    match sqlx::query!(
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
    {
        Ok(_) => Ok(()),
        Err(err) => match err.as_database_error().and_then(|db_err| db_err.code()) {
            // PostgreSQL specific code for unique violation
            Some(code) if code == "23505" => Err(Error::EmailInUseError),
            _ => Err(Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to insert user into database".into(),
            )),
        },
    }
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "updating user details in database", skip(state, user))]
pub async fn update_user(state: &AppState, user: &User, user_id: &Uuid) -> Result<()> {
    match sqlx::query!(
        r#"
        UPDATE users 
        SET name = $1, email = $2, password_hash = $3, role = $4, version = version + 1
        WHERE id = $5 AND version = $6
        RETURNING version
        "#,
        user.name,
        user.email,
        user.password.expose_secret(),
        user.role.clone() as UserRole,
        user_id,
        user.version
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(Error::EditConflictError),
            _ => Err(Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to update user in database".into(),
            )),
        },
    }
}
// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "deleting user from database", skip(state, user_id))]
pub async fn delete_user(state: &AppState, user_id: &Uuid) -> Result<()> {
    match sqlx::query!(
        r#"
        DELETE FROM users 
        WHERE id = $1
        RETURNING id
        "#,
        user_id,
    )
    .fetch_one(&state.db_pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(Error::NotFoundError),
            _ => Err(Error::UnexpectedError(
                std::sync::Arc::new(err),
                "Failed to delete user from database".into(),
            )),
        },
    }
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
