use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use serde::Deserialize;

use crate::AppState;

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "create_post_button.html")]
pub struct CreatePostButtonTemplate {}

#[derive(Deserialize)]
pub struct PathData {
    generation_id: i64,
}

pub async fn handle(
    State(s): State<AppState>,
    Path(p): Path<PathData>,
) -> Result<CreatePostButtonTemplate, WebError> {
    tracing::info!("Creating new post: {}", p.generation_id);
    s.db.write_post(p.generation_id).await?;
    Ok(CreatePostButtonTemplate {})
}
