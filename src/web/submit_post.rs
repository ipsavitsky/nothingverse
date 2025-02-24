use std::time::{SystemTime, UNIX_EPOCH};

use askama::Template;
use axum::{Form, extract::State};
use serde::Deserialize;

use crate::AppState;

#[derive(Template)]
#[template(path = "create_post_button.html")]
pub struct CreatePostButtonTemplate {}

#[derive(Deserialize)]
pub struct PostData {
    content: String,
}

#[axum::debug_handler]
pub async fn handle(State(s): State<AppState>, Form(f): Form<PostData>) -> CreatePostButtonTemplate {
    tracing::info!("Creating new post: {}", f.content);
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let _ = sqlx::query!(
        "INSERT INTO posts (post, timestamp) VALUES (?, ?)",
        f.content,
        time
    ).execute(&s.db_pool).await;
    CreatePostButtonTemplate {}
}
