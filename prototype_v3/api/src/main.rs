use api::{web::server, Config};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Config::default();

    let tcp_listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.server_host, config.server_port))
            .await
            .unwrap();

    println!(
        "->> {:<12} - {}",
        "LISTENING",
        tcp_listener.local_addr().unwrap()
    );

    server::serve(tcp_listener).await.unwrap()
}
