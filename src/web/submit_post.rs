use askama::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::AppState;

#[derive(Template)]
#[template(path = "create_post_button.html")]
pub struct CreatePostButtonTemplate {}

#[derive(Deserialize)]
pub struct PostData {
    generation_id: i64,
}

pub async fn handle(
    State(s): State<AppState>,
    Form(f): Form<PostData>,
) -> CreatePostButtonTemplate {
    tracing::info!("Creating new post: {}", f.generation_id);
    let _ = sqlx::query!(
        "INSERT INTO posts (generation_id) VALUES (?)",
        f.generation_id,
    )
    .execute(&s.db_pool)
    .await;
    CreatePostButtonTemplate {}
}
