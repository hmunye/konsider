use axum::routing::post;
use axum::Router;

use crate::web::server::AppState;

use super::create_user;

pub fn admin_routes(state: AppState) -> Router {
    Router::new()
        .route("/create-user", post(create_user))
        .with_state(state)
}
