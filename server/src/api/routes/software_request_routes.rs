use axum::routing::{delete, get};
use axum::Router;

use crate::api::controllers::{
    api_create_software_request, api_delete_software_request, api_get_all_software_requests,
    api_update_software_request,
};
use crate::server::ServerState;

pub fn software_request_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/requests` path
    Router::new()
        .route(
            "/",
            get(api_get_all_software_requests).post(api_create_software_request),
        )
        .route(
            "/:request_id",
            delete(api_delete_software_request).patch(api_update_software_request),
        )
}
