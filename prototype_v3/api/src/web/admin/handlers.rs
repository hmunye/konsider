use axum::routing::post;
use axum::Router;

use crate::server::AppState;

use super::api_create_user;

pub fn admin_routes(state: AppState) -> Router {
    Router::new()
        .route("/create-user", post(api_create_user))
        .with_state(state)
}
