use axum::routing::IntoMakeService;
use axum::serve::Serve;
use axum::Router;
use secrecy::SecretString;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::api::{auth_routes, health_routes};
use crate::config::DatabaseConfig;
use crate::{Config, Result};

// Type alias for axum's serve
type ServeType = Serve<IntoMakeService<Router>, Router>;

#[derive(Debug)]
pub struct Server {
    port: u16,
    instance: ServeType,
}

impl Server {
    // Build a new server instance
    pub async fn build(config: Config) -> Result<Server> {
        let db_pool = get_db_pool(&config.database)?;

        let bind = format!("{}:{}", config.server.host, config.server.port);

        let tcp_listener = tokio::net::TcpListener::bind(&bind)
            .await
            .expect("failed to create tcp listener from provided address");

        // Grab the assigned port from `tcp_listener`
        let port = tcp_listener
            .local_addr()
            .expect("failed to get local address bound to tcp listener")
            .port();

        let instance = serve(tcp_listener, db_pool, config.server.jwt_secret).await?;

        tracing::info!(
            "{}",
            format_args!("[LISTENING ON - {}:{}]", &config.server.host, &port)
        );

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

pub fn get_db_pool(config: &DatabaseConfig) -> Result<PgPool> {
    // Using PgPool allows for concurrency by borrowing a PgConncection from the pool for executing queries
    Ok(PgPoolOptions::new()
        // Set the minimum number of connections to maintain at all times
        .min_connections(10)
        // Set the maximum number of connections that this pool should maintain
        .max_connections(20)
        // The amount of time the pool will wait to acquire a connection
        .acquire_timeout(std::time::Duration::from_secs(10))
        // The amount of time a connection can stay idle in the pool before it is closed
        .idle_timeout(std::time::Duration::from_secs(900)) // Set to 15 minutes
        .connect_lazy_with(config.connect_options()))
}

#[derive(Clone)]
pub struct ServerState {
    pub db_pool: PgPool,
    pub jwt_secret: SecretString,
}

pub async fn serve(
    tcp_listener: tokio::net::TcpListener,
    db_pool: PgPool,
    jwt_secret: SecretString,
) -> Result<ServeType> {
    let state = ServerState {
        db_pool,
        jwt_secret,
    };

    let server = Router::new().nest(
        "/api/v1",
        Router::<ServerState>::new()
            .nest("/health", health_routes())
            .nest("/auth", auth_routes())
            .with_state(state),
    );

    Ok(axum::serve(tcp_listener, server.into_make_service()))
}
