use konsider_api::http;

pub struct TestServer {
    pub addr: String,
}

pub async fn spawn_server() -> TestServer {
    let addr = format!("{}:0", "127.0.0.1");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    let port = listener.local_addr().unwrap().port();
    let server = http::serve(listener).expect("Failed to start test server");

    tokio::spawn(async move { server.await });

    let addr = format!("{}:{}", "http://127.0.0.1", port);

    TestServer { addr }
}
