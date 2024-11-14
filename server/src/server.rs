use axum::routing::IntoMakeService;
use axum::serve::Serve;
use axum::Router;

use crate::api::health_routes;
use crate::Result;

// Type alias for axum's serve
type ServeType = Serve<IntoMakeService<Router>, Router>;

#[derive(Debug)]
pub struct Server {
    port: u16,
    instance: ServeType,
}

impl Server {
    // Build a new server instance
    pub async fn build(bind: &str) -> Result<Server> {
        let tcp_listener = tokio::net::TcpListener::bind(&bind)
            .await
            .expect("failed to create tcp listener from provided address");

        // Grab the assigned port from `tcp_listener`
        let port = tcp_listener
            .local_addr()
            .expect("failed to get local address bound to tcp listener")
            .port();

        let instance = serve(tcp_listener).await?;

        println!(">> LISTENING ON {bind}");

        Ok(Self { port, instance })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // Start the server and await its completion
    pub async fn run(self) -> Result<()> {
        self.instance.await.expect("failed to run server");

        Ok(())
    }
}

pub async fn serve(tcp_listener: tokio::net::TcpListener) -> Result<ServeType> {
    let server = Router::new().nest("/api/v1", Router::new().nest("/health", health_routes()));

    Ok(axum::serve(tcp_listener, server.into_make_service()))
}
