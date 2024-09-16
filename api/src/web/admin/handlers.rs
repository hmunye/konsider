use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::server::AppState;
use crate::web::admin::api_create_user;

use super::{api_delete_user, api_get_all_users, api_get_user, api_update_user};

// ---------------------------------------------------------------------------------------------------------------
pub fn admin_routes(state: AppState) -> Router {
    Router::new()
        .route("/users", get(api_get_all_users))
        .route("/users/:user_id", get(api_get_user))
        .route("/users", post(api_create_user))
        .route("/users/:user_id", patch(api_update_user))
        .route("/users/:user_id", delete(api_delete_user))
        .with_state(state)
}
