use axum::routing::post;
use axum::Router;

use crate::web::server::AppState;

use super::login;

pub fn auth_routes(state: AppState) -> Router {
    Router::new().route("/login", post(login)).with_state(state)
}
