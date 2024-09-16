use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn health_check_successful() {
    let server = spawn_server().await;
    let health_check_url = format!("{}/v1/health-check", server.addr);

    let response = server.get_request(&health_check_url, None, None).await;
    assert_eq!(200, response.status().as_u16());
}
