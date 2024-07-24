use crate::http::AppState;
use axum::{extract::State, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    id: i32,
    title: String,
    content: String,
    published: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
pub enum ApiResponse {
    Posts(Vec<Post>),
    Error(ErrorResponse),
}

pub async fn get_posts(State(state): State<AppState>) -> Json<ApiResponse> {
    match sqlx::query!(
        r#"
        SELECT id, title, content, published
        FROM posts
        "#
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(records) => {
            let posts: Vec<Post> = records
                .into_iter()
                .map(|record| Post {
                    id: record.id,
                    title: record.title.clone(),
                    content: record.content.clone(),
                    published: record.published,
                })
                .collect();
            Json(ApiResponse::Posts(posts))
        }
        Err(err) => {
            let error = ErrorResponse {
                error: err.to_string(),
            };
            Json(ApiResponse::Error(error))
        }
    }
}
