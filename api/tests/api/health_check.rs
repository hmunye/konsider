use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn health_check_test() {
    let server = spawn_server().await;
    let url = format!("{}/health-check", server.addr);

    // 1. Health Check Request
    let response = server.get_request(&url).await;

    assert_eq!(200, response.status().as_u16());
}
