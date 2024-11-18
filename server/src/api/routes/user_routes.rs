use axum::routing::post;
use axum::Router;

use crate::api::controllers::api_change_password;
use crate::server::ServerState;

pub fn user_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/users` path
    Router::new().route("/password", post(api_change_password))
}
