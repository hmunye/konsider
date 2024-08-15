use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, IntoMakeService};
use axum::serve::Serve;
use axum::Router;

use super::auth::auth_routes;

type Server = Serve<IntoMakeService<Router>, Router>;

async fn health_check_handler() -> impl IntoResponse {
    (StatusCode::OK, "status: ok")
}

pub fn serve(tcp_listener: tokio::net::TcpListener) -> Server {
    let routes_all = Router::new()
        .route("/health-check", get(health_check_handler))
        .nest("/auth", auth_routes());

    axum::serve(tcp_listener, routes_all.into_make_service())
}
