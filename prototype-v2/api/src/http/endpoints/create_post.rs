use crate::http::AppState;
use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PostData {
    title: String,
    content: String,
}

pub async fn create_post(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<PostData>,
) -> impl IntoResponse {
    match sqlx::query!(
        r#"
        INSERT INTO posts (title, content)
        VALUES ($1, $2)
        "#,
        payload.title,
        payload.content,
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
