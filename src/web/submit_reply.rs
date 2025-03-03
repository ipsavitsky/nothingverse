use askama::Template;
use axum::{
    extract::{Path, State},
    Form,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Template)]
#[template(path = "create_reply_button.html")]
pub struct CreateReplyButtonTemplate {
    post_id: i64,
}

#[derive(Deserialize)]
pub struct PostData {
    content: String,
}

#[derive(Deserialize)]
pub struct PathData {
    post_id: i64,
}

pub async fn handle(
    Path(p): Path<PathData>,
    State(s): State<AppState>,
    Form(f): Form<PostData>,
) -> CreateReplyButtonTemplate {
    tracing::info!("Creating new reply to post: {}", f.content);
    let _ = sqlx::query!(
        "INSERT INTO replies (content, post_id) VALUES (?, ?)",
        f.content,
        p.post_id
    )
    .execute(&s.db_pool)
    .await;
    CreateReplyButtonTemplate { post_id: p.post_id }
}
