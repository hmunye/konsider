use axum::routing::post;
use axum::Router;

use crate::api::controllers::api_login;
use crate::server::ServerState;

pub fn auth_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/auth` path
    Router::new().route("/login", post(api_login))
}