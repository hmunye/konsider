use axum::extract::{self, State};
use axum::http::StatusCode;
use axum::response::Response;
use serde_json::json;

use crate::web::server::AppState;
use crate::{Error, Result, User, UserRole};

#[tracing::instrument(
    name = "Creating new user", 
    // Won't include in logs
    skip(state, payload),
    fields(
        user_email = %payload.email,
        user_name = %payload.name
    )
)]
pub async fn api_create_user(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<User>,
) -> Result<Response<String>> {
    // Validate payload
    payload.parse().map_err(|err| {
        tracing::error!("New user details are invalid");
        err
    })?;

    // TODO: Hash and salt password
    let password_hash = &payload.password;

    insert_user(&state, &payload, password_hash.to_string()).await?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "success": true,
            })
            .to_string(),
        )
        .unwrap();

    Ok(response)
}

#[tracing::instrument(name = "Inserting user in DB", skip(state, payload, password_hash))]
async fn insert_user(state: &AppState, payload: &User, password_hash: String) -> Result<()> {
    match sqlx::query!(
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
    {
        Ok(..) => {
            tracing::info!("New user details have been saved",);
            Ok(())
        }
        Err(err) => {
            tracing::error!("Failed to execute query: {:?}", err);
            Err(Error::DatabaseError(err.to_string()))
        }
    }
}
