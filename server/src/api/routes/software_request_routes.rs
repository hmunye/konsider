use axum::routing::get;
use axum::Router;

use crate::api::controllers::{api_create_software_request, api_get_all_software_requests};
use crate::server::ServerState;

pub fn software_request_routes() -> Router<ServerState> {
    // All routes are under the `/api/v1/requests` path
    Router::new().route(
        "/",
        get(api_get_all_software_requests).post(api_create_software_request),
    )
}
