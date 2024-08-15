use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, IntoMakeService};
use axum::serve::Serve;
use axum::Router;
use sqlx::PgPool;

use super::admin::admin_routes;
use super::auth::auth_routes;

type Server = Serve<IntoMakeService<Router>, Router>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

async fn health_check() -> impl IntoResponse {
    println!("->> {:<12} - health_check", "HANDLER");
    (StatusCode::OK, "status: ok")
}

pub fn serve(tcp_listener: tokio::net::TcpListener, db_pool: PgPool) -> Server {
    let state = AppState { db_pool };

    let routes_all = Router::new()
        .route("/health-check", get(health_check))
        .nest("/auth", auth_routes(state.clone()))
        .nest("/admin", admin_routes(state.clone()));

    axum::serve(tcp_listener, routes_all.into_make_service())
}
