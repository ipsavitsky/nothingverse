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
    let gen_group_used = s.db.group_is_used(p.generation_id).await?;
    tracing::debug!("Generation is used: {}", gen_group_used);
    if !gen_group_used {
        s.db.write_post(p.generation_id).await?;
        Ok(CreatePostButtonTemplate {})
    } else {
        Err(WebError::GenerationAlreadyUsed)
    }
}
