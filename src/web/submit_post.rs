use askama::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::AppState;

#[derive(Template)]
#[template(path = "create_post_button.html")]
pub struct CreatePostButtonTemplate {}

#[derive(Deserialize)]
pub struct PostData {
    content: String,
}

pub async fn handle(
    State(s): State<AppState>,
    Form(f): Form<PostData>,
) -> CreatePostButtonTemplate {
    tracing::info!("Creating new post: {}", f.content);
    let _ = sqlx::query!("INSERT INTO posts (content) VALUES (?)", f.content,)
        .execute(&s.db_pool)
        .await;
    CreatePostButtonTemplate {}
}
