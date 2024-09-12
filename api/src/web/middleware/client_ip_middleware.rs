use axum::extract::ConnectInfo;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

// Extracts client_ip from X_FORWARDED_FOR header from Nginx reverse proxy or ConnectInfo extension
pub async fn extract_client_ip(mut request: Request, next: Next) -> Response {
    let client_ip = if let Some(x_forwarded_for) = request.headers().get("x-forwarded-for") {
        x_forwarded_for.to_str().unwrap_or("N/A").to_string()
    } else if let Some(connect_info) = request
        .extensions()
        .get::<ConnectInfo<std::net::SocketAddr>>()
    {
        connect_info.ip().to_string()
    } else {
        "N/A".to_string()
    };

    request.extensions_mut().insert(client_ip.clone());

    next.run(request).await
}
