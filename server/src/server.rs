use axum::http::StatusCode;
use axum::routing::{get, IntoMakeService};
use axum::serve::Serve;
use axum::Router;

use crate::Result;

// Type alias for axum's serve
type ServeType = Serve<IntoMakeService<Router>, Router>;

#[derive(Debug)]
pub struct Server {
    instance: ServeType,
}

impl Server {
    // Build a new server instance
    pub async fn build() -> Result<Server> {
        let bind = "127.0.0.1:8080";

        let tcp_listener = tokio::net::TcpListener::bind(&bind)
            .await
            .expect("failed to create tcp listener from provided address");

        let instance = serve(tcp_listener).await?;

        println!(">> LISTENING ON {bind}");

        Ok(Self { instance })
    }

    // Start the server and await its completion
    pub async fn run(self) -> Result<()> {
        self.instance.await.expect("failed to run server");

        Ok(())
    }
}

pub async fn serve(tcp_listener: tokio::net::TcpListener) -> Result<ServeType> {
    let server = Router::new().nest(
        "/api/v1",
        Router::new().route("/health", get(api_health_check)),
    );

    Ok(axum::serve(tcp_listener, server.into_make_service()))
}

async fn api_health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}
