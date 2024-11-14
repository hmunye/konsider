use axum::routing::get;
use axum::Router;

use crate::api::controllers::api_health_check;

pub fn health_routes() -> Router {
    // All routes are under the `/api/v1/health` path
    Router::new().route("/", get(api_health_check))
}
