use axum::{routing::get, Router};

mod health_check;

pub use health_check::*;

use super::AppState;

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/v1/healthcheck", get(health_check))
        .with_state(state.clone())
}
