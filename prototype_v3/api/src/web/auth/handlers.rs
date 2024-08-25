use axum::routing::post;
use axum::Router;

use crate::server::AppState;
use crate::web::auth::api_login;

// ---------------------------------------------------------------------------------------------------------------
pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(api_login))
        .with_state(state)
}
