use axum::http::StatusCode;
use axum::response::IntoResponse;
use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::{Config, Environment};

// ---------------------------------------------------------------------------------------------------------------
#[tracing::instrument(name = "health check")]
pub async fn health_check() -> impl IntoResponse {
    let environment: Environment = std::env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to convert ENVIRONMENT env variable");

    let version: String = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "N/A".into());

    let config = Config::default();

    let connection_string = config.connection_string();

    let _ = PgPool::connect(connection_string.expose_secret())
        .await
        .expect("Failed to connect to database from health_check endpoint");

    (
        StatusCode::OK,
        format!(
            "{{\n\tstatus: available,\n\tenvironment: {},\n\tversion: {}\n}}",
            environment.as_str(),
            version
        ),
    )
        .into_response()
}
