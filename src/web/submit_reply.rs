use askama::Template;
use askama_web::WebTemplate;
use axum::{
    extract::{Path, State},
    Form,
};
use serde::Deserialize;

use crate::AppState;

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "create_reply_button.html")]
pub struct CreateReplyButtonTemplate {
    post_id: i64,
}

#[derive(Deserialize)]
pub struct PostData {
    generation_id: i64,
}

#[derive(Deserialize)]
pub struct PathData {
    post_id: i64,
}

pub async fn handle(
    Path(p): Path<PathData>,
    State(s): State<AppState>,
    Form(f): Form<PostData>,
) -> Result<CreateReplyButtonTemplate, WebError> {
    tracing::info!("Creating new reply to post: {}", f.generation_id);
    s.db.write_reply(f.generation_id, p.post_id).await?;
    Ok(CreateReplyButtonTemplate { post_id: p.post_id })
}
