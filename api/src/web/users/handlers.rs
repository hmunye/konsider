use axum::routing::get;
use axum::Router;

use crate::server::AppState;

use crate::web::reject_non_admin_users;
use crate::web::users::{
    api_create_user, api_delete_user, api_get_all_users, api_get_user, api_update_user,
};

// ---------------------------------------------------------------------------------------------------------------
pub fn users_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(api_get_all_users).post(api_create_user))
        .route(
            "/:user_id",
            get(api_get_user)
                .patch(api_update_user)
                .delete(api_delete_user),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            reject_non_admin_users,
        ))
        .with_state(state)
}
