use axum::routing::post;
use axum::Router;

use super::login_handler;

pub fn auth_routes() -> Router {
    Router::new().route("/login", post(login_handler))
}
