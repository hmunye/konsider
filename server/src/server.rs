use axum::http::{header, Method, Request};
use axum::Router;
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use secrecy::SecretString;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio_native_tls::{
    native_tls::{Identity, Protocol, TlsAcceptor as NativeTlsAcceptor},
    TlsAcceptor,
};
use tower::ServiceBuilder;
use tower_http::classify::StatusInRangeAsFailures;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_service::Service;

use crate::api::{
    auth_routes, health_routes, main_response_mapper, requester_routes, software_request_routes,
    software_review_routes, software_routes, user_routes, TokenCache,
};
use crate::config::{Config, DatabaseConfig};
use crate::Result;

#[derive(Debug)]
pub struct Server {
    port: u16,
    instance: Router,
    environment: String,
}

impl Server {
    // Build a new server instance
    pub async fn build(
        config: Config,
        token_cache: TokenCache,
    ) -> Result<(Server, tokio::net::TcpListener)> {
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

        let instance = setup_server(db_pool, config.server.jwt_secret, token_cache).await?;

        tracing::info!(
            "{}",
            format_args!("[LISTENING ON - {}:{}]", &config.server.host, &port)
        );

        Ok((
            Self {
                port,
                instance,
                environment: config.server.environment,
            },
            tcp_listener,
        ))
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // Start the server and await its completion, either HTTP or HTTPS
    pub async fn run(self, tcp_listener: tokio::net::TcpListener) -> Result<()> {
        match self.environment.as_str() {
            "production" => self.run_https(tcp_listener).await,
            _ => {
                axum::serve(tcp_listener, self.instance.into_make_service())
                    .await
                    .expect("failed to start HTTP server");

                Ok(())
            }
        }
    }

    async fn run_https(&self, tcp_listener: tokio::net::TcpListener) -> Result<()> {
        let tls_acceptor = native_tls_acceptor(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("certs")
                .join("server.key"),
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("certs")
                .join("server.crt"),
        );

        let tls_acceptor = TlsAcceptor::from(tls_acceptor);

        futures_util::pin_mut!(tcp_listener);

        loop {
            let (cnx, addr) = tcp_listener.accept().await.unwrap();
            let tower_service = self.instance.clone();
            let tls_acceptor = tls_acceptor.clone();

            tokio::spawn(async move {
                let Ok(stream) = tls_acceptor.accept(cnx).await else {
                    tracing::error!("error during TLS handshake connection from {}", addr);
                    return;
                };

                let stream = TokioIo::new(stream);

                let hyper_service =
                    hyper::service::service_fn(move |request: Request<Incoming>| {
                        tower_service.clone().call(request)
                    });

                let ret = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                    .serve_connection_with_upgrades(stream, hyper_service)
                    .await;

                if let Err(err) = ret {
                    tracing::warn!("error serving connection from {}: {}", addr, err);
                }
            });
        }
    }
}

fn native_tls_acceptor(
    key_file: std::path::PathBuf,
    cert_file: std::path::PathBuf,
) -> NativeTlsAcceptor {
    let key = std::fs::read_to_string(&key_file).unwrap();

    let cert = std::fs::read_to_string(&cert_file).unwrap();

    let id = Identity::from_pkcs8(cert.as_bytes(), key.as_bytes()).unwrap();

    NativeTlsAcceptor::builder(id)
        .min_protocol_version(Some(Protocol::Tlsv12))
        .build()
        .unwrap()
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
    pub token_cache: TokenCache,
}

pub async fn setup_server(
    db_pool: PgPool,
    jwt_secret: SecretString,
    token_cache: TokenCache,
) -> Result<Router> {
    let state = ServerState {
        db_pool,
        jwt_secret,
        token_cache,
    };

    let origin = ["http://localhost:3030".parse().unwrap()];

    let cors_layer = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_credentials(true)
        .allow_headers([
            header::ACCEPT,
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            header::CONTENT_TYPE,
        ]);

    let server = Router::new()
        .nest(
            "/api/v1",
            Router::<ServerState>::new()
                .nest("/health", health_routes())
                .nest("/auth", auth_routes())
                .nest("/users", user_routes())
                .nest("/requesters", requester_routes())
                .nest("/software", software_routes())
                .nest("/requests", software_request_routes())
                .nest("/reviews", software_review_routes())
                .with_state(state),
        )
        .layer(axum::middleware::map_response(main_response_mapper))
        .layer(CompressionLayer::new())
        .layer(cors_layer)
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new(
                    // By default, the `new_for_http` method for TraceLayer
                    // only classifies `5xx` errors as failures.
                    // Now, any error with status from 400 to 599 is classified as an error
                    StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
                )
                .make_span_with(|request: &Request<_>| {
                    let request_id = uuid::Uuid::new_v4().to_string();

                    // Will be included with every request log
                    tracing::span!(
                        tracing::Level::INFO,
                        "request",
                        %request_id,
                        method = ?request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                }),
            ),
        );

    Ok(server)
}
