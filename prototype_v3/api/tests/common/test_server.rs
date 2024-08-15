use api::web::server;

use api::Config;

pub struct TestServer {
    pub addr: String,
}

pub async fn spawn_server() -> TestServer {
    dotenvy::dotenv().ok();

    let config = Config::default();

    // Using port '0' will trigger the OS to scan for an available port
    // This allows the server to continue running on port 8000 while each test is executed using
    // a different port. Avoids port conflicts
    let addr = format!("{}:0", config.server_host);

    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let port = tcp_listener.local_addr().unwrap().port();

    let server = server::serve(tcp_listener);

    // Spawns a new asynchronous task
    // Used to start a new task that starts a new instance of the server
    tokio::spawn(async move { server.await.unwrap() });

    let addr = format!("http://{}:{}", config.server_host, port);

    TestServer { addr }
}
