// TODO: Add middleware that limits the size of request bodies by default
// Reject any input over 4 GiB or any input that could _encode_ to a string longer than 4 GiB

use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::extract::ConnectInfo;
use axum::http::{header, Method, Request};
use axum::middleware::AddExtension;
use axum::routing::get;
use axum::serve::Serve;
use axum::Router;
use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use time::Duration;
use tower::ServiceBuilder;
use tower_cookies::cookie::SameSite;
use tower_http::classify::StatusInRangeAsFailures;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::{fred::prelude::*, RedisStore};
use tracing::Level;
use uuid::Uuid;

use crate::web::{
    admin_routes, auth_routes, health_check, main_response_mapper, reject_non_admin_users,
};
use crate::Config;

type Server = Serve<
    IntoMakeServiceWithConnectInfo<Router, std::net::SocketAddr>,
    AddExtension<Router, ConnectInfo<std::net::SocketAddr>>,
>;

// ---------------------------------------------------------------------------------------------------------------
pub struct Application {
    port: u16,
    host: String,
    server: Server,
}

impl Application {
    pub async fn build(config: Config) -> crate::Result<Self> {
        let db_pool = get_db_pool(&config);

        let redis_pool = get_redis_pool(&config.redis_uri()).await;

        let session_store = RedisStore::new(redis_pool);

        let addr = format!("{}:{}", config.server_host, config.server_port);

        let tcp_listener = tokio::net::TcpListener::bind(&addr)
            .await
            .expect("Failed to bind to address");

        let host = config.server_host;

        let port = tcp_listener
            .local_addr()
            .expect("Failed to get local address from tcp listener")
            .port();

        let server = serve(tcp_listener, db_pool, session_store).await?;

        Ok(Self { port, host, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub async fn run_server(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

// ---------------------------------------------------------------------------------------------------------------
pub fn get_db_pool(config: &Config) -> PgPool {
    // Using PgPool allow concurrency by borrowing a PgConncection from the pool for executing queries
    PgPoolOptions::new()
        // Set the minimum number of connections to maintain at all times
        .min_connections(10)
        // Set the maximum number of connections that this pool should maintain.
        .max_connections(20)
        // The amount of time the pool will wait to aquire a connection
        .acquire_timeout(std::time::Duration::from_secs(10))
        // The amount of time a connection can stay idle in the pool before it is closed
        // Set to 15 minutes
        .idle_timeout(std::time::Duration::from_secs(900))
        // Won't connect until a query is made
        .connect_lazy(config.connection_string().expose_secret())
        .expect("Failed to create connection pool")
}
// ---------------------------------------------------------------------------------------------------------------
pub async fn get_redis_pool(redis_uri: &Secret<String>) -> RedisPool {
    let redis_config = RedisConfig::from_url(redis_uri.expose_secret())
        .expect("Failed to get redis config from uri");

    let redis_pool =
        RedisPool::new(redis_config, None, None, None, 6).expect("Failed to create redis pool");

    redis_pool.connect();

    redis_pool
        .wait_for_connect()
        .await
        .expect("Failed to connect to redis server");

    redis_pool
}
// ---------------------------------------------------------------------------------------------------------------
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub async fn serve(
    tcp_listener: tokio::net::TcpListener,
    db_pool: PgPool,
    session_store: RedisStore<RedisPool>,
) -> crate::Result<Server> {
    let state = AppState { db_pool };

    // User ID will be stored into the session state on login and will be retrieved
    // on other endpoints when specified
    //
    // SessionManager checks for session cookies in incoming requests, loads
    // corresponding state from the session store, handles cookie properties
    // (Does all the heavy lifting)
    //
    // SessionManagerLayer configures cookie properties
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_http_only(true)
        .with_name("id")
        .with_domain("localhost")
        .with_same_site(SameSite::Lax)
        .with_path("/")
        .with_expiry(Expiry::OnInactivity(Duration::minutes(15)));

    let origin = ["http://localhost:3000".parse().unwrap()];

    // TODO: Update to be more strict
    let cors_layer = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_credentials(true)
        .allow_headers([
            header::ACCEPT,
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
        ]);
    let routes_all = Router::new()
        .route("/v1/health-check", get(health_check))
        .nest("/v1/auth", auth_routes(state.clone()))
        .nest(
            "/v1/admin",
            admin_routes(state.clone()).route_layer(axum::middleware::from_fn_with_state(
                state.clone(),
                reject_non_admin_users,
            )),
        )
        .layer(axum::middleware::map_response(main_response_mapper))
        .layer(session_layer)
        .layer(cors_layer)
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new(
                    // By default, the 'new_for_http' method for TraceLayer only
                    // classifies 5xx errors as failures
                    // Here, any error with status from 400 to 599 is classified as an error
                    StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
                )
                .make_span_with(|request: &Request<_>| {
                    let request_id = Uuid::new_v4().to_string();

                    let client_ip = if let Some(client_connection) = request
                        .extensions()
                        .get::<ConnectInfo<std::net::SocketAddr>>()
                    {
                        Ok(client_connection.ip())
                    } else {
                        Err("N/A".to_string())
                    };

                    // Will be included with every request log
                    tracing::span!(
                        Level::INFO,
                        "request",
                        %request_id,
                        client_ip = {
                            if client_ip.is_ok() {
                                client_ip.unwrap().to_string()
                            } else {
                                format!("{:?}", client_ip)
                            }
                        },
                        method = ?request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                }),
            ),
        );

    Ok(axum::serve(
        tcp_listener,
        routes_all.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    ))
}
