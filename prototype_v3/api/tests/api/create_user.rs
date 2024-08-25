use serde_json::json;

use crate::common::spawn_server;

// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_returns_200_status() {
    let server = spawn_server().await;
    let url = format!("{}/admin/create-user", server.addr);

    let email: String = String::from("john@gmail.com");

    // Payload
    let body = json!({
        "name": "John",
        "email": &email,
        "password": "password123",
        "role": "Reviewer"
    });

    // Request
    let response = server.post_request(&url, body.to_string()).await;

    // Check if user has been created
    let _ = sqlx::query!(
        r#"
        SELECT id
        FROM "user"
        WHERE email=$1 
        "#,
        email
    )
    .fetch_one(&server.db_pool)
    .await
    .unwrap();

    assert_eq!(200, response.status().as_u16());
}
// ---------------------------------------------------------------------------------------------------------------
//#[tokio::test]
//// Returns 422 because the payload can't be deserialized into the 'User' struct
//async fn create_user_returns_422_status() {
//    let server = spawn_server().await;
//    let url = format!("{}/admin/create-user", server.addr);
//
//    // Payloads where the user should not be created
//    let test_cases = vec![
//        (
//            json!({
//                "email": "test@gmail.com",
//                "password": "testing123",
//                "role": "Reviewer",
//            }),
//            "missing name.",
//        ),
//        (
//            json!({
//                "name": "John",
//                "password": "testing123",
//                "role": "Reviewer",
//            }),
//            "missing email.",
//        ),
//        (
//            json!({
//                "name": "John",
//                "email": "test@gmail.com",
//                "role": "Reviewer",
//            }),
//            "missing password.",
//        ),
//        (
//            json!({
//                "name": "John",
//                "email": "test@gmail.com",
//                "password": "testing123",
//            }),
//            "missing role.",
//        ),
//    ];
//
//    // Requests
//    for (invalid_body, error_message) in test_cases {
//        let response = server.post_request(&url, invalid_body.to_string()).await;
//
//        (
//            assert_eq!(422, response.status().as_u16()),
//            "API did not fail with a 422 status when the payload was {}",
//            error_message,
//        );
//    }
//}
// ---------------------------------------------------------------------------------------------------------------
#[tokio::test]
async fn create_user_returns_400_status() {
    let server = spawn_server().await;
    let url = format!("{}/admin/create-user", server.addr);

    // Payloads where the user should not be created
    let test_cases = vec![
        (
            json!({
                "name": "",
                "email": "test@gmail.com",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "empty name.",
        ),
        (
            json!({
                "name": "John",
                "email": "",
                "password": "testing123",
                "role": "Reviewer",
            }),
            "empty email",
        ),
        (
            json!({
                "name": "John",
                "email": "test@gmail.com",
                "password": "",
                "role": "Reviewer",
            }),
            "empty password",
        ),
        (
            json!({
                "name": "John",
                "email": "//$(test@gmail.com)",
                "password": "",
                "role": "Reviewer",
            }),
            "malformed email",
        ),
        // Would return a 422 normally status because role is from the UserRole enum
        //        (
        //            json!({
        //                "name": "John",
        //                "email": "test@gmail.com",
        //                "password": "testing123",
        //                "role": "",
        //            }),
        //            "empty role",
        //        ),
    ];

    // Requests
    for (invalid_body, error_message) in test_cases {
        let response = server.post_request(&url, invalid_body.to_string()).await;

        (
            assert_eq!(400, response.status().as_u16()),
            "API did not fail with a 400 status when the payload was {}",
            error_message,
        );
    }
}
