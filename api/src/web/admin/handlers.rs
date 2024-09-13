use axum::routing::{delete, patch, post};
use axum::Router;

use crate::server::AppState;
use crate::web::admin::api_create_user;

use super::{api_delete_user, api_update_user};

// ---------------------------------------------------------------------------------------------------------------
pub fn admin_routes(state: AppState) -> Router {
    Router::new()
        .route("/create-user", post(api_create_user))
        .route("/update-user/:user_id", patch(api_update_user))
        .route("/delete-user/:user_id", delete(api_delete_user))
        .with_state(state)
}
