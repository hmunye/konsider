// TODO: Add middleware that limits the size of request bodies by default
// Reject any input over 4 GiB,
// or any input that could _encode_ to a string longer than 4 GiB

use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, IntoMakeService};
use axum::serve::Serve;
use axum::Router;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::Level;

use super::admin::admin_routes;
use super::auth::auth_routes;

type Server = Serve<IntoMakeService<Router>, Router>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[tracing::instrument(name = "Health Check")]
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "status: ok")
}

pub fn serve(tcp_listener: tokio::net::TcpListener, db_pool: PgPool) -> Server {
    let state = AppState { db_pool };

    let routes_all = Router::new()
        .route("/health-check", get(health_check))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/admin", admin_routes(state.clone()))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let request_id = uuid::Uuid::new_v4().to_string();

                // Will be included with every request
                tracing::span!(
                    Level::DEBUG,
                    "request",
                    %request_id,
                    method = ?request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                )
            }),
        );

    axum::serve(tcp_listener, routes_all.into_make_service())
}
