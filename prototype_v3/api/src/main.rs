use api::web::server;

#[tokio::main]
async fn main() {
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:1234")
        .await
        .unwrap();

    println!(
        "->> {:<12} - {}",
        "LISTENING",
        tcp_listener.local_addr().unwrap()
    );

    server::serve(tcp_listener).await.unwrap()
}
