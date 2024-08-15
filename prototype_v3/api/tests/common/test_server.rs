use api::web::server;

pub struct TestServer {
    pub addr: String,
}

pub async fn spawn_server() -> TestServer {
    // Using port '0' will trigger the OS to scan for an available port
    // This allows the server to continue running on port 8000 while each test is executed using
    // a different port. Avoids port conflicts
    let addr = format!("{}:0", "127.0.0.1");

    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let port = tcp_listener.local_addr().unwrap().port();

    let server = server::serve(tcp_listener);

    // Spawns a new asynchronous task
    // Used to start a new task that starts a new instance of the server
    tokio::spawn(async move { server.await.unwrap() });

    let addr = format!("http://{}:{}", "127.0.0.1", port);

    TestServer { addr }
}
