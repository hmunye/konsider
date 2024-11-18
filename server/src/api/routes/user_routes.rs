use axum::routing::{get, post};
use axum::Router;

use crate::api::controllers::{api_change_password, api_get_all_users};
use crate::server::ServerState;

pub fn user_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/users` path
    Router::new()
        .route("/", get(api_get_all_users))
        .route("/password", post(api_change_password))
}
