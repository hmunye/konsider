mod endpoints;

use axum::{routing::IntoMakeService, serve::Serve, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn serve(
    listener: TcpListener,
    db: PgPool,
) -> Result<Serve<IntoMakeService<Router>, Router>, std::io::Error> {
    let state = AppState { db };

    let server = axum::serve(listener, endpoints::api_router(state).into_make_service());

    Ok(server)
}
