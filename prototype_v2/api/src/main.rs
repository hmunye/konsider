use konsider_api::http;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Failed to bind to address");

    println!("->> LISTENING on {}", listener.local_addr().unwrap());

    http::serve(listener)?.await
}
