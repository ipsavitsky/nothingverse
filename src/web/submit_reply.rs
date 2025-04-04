use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use serde::Deserialize;

use crate::AppState;

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "create_reply_button.html")]
pub struct CreateReplyButtonTemplate {
    post_id: i64,
    generated_reply: Option<String>,
}

#[derive(Deserialize)]
pub struct PathData {
    post_id: i64,
    generation_id: i64,
}

pub async fn handle(
    Path(p): Path<PathData>,
    State(s): State<AppState>,
) -> Result<CreateReplyButtonTemplate, WebError> {
    tracing::info!("Creating new reply to post: {}", p.generation_id);
    let gen_group_used = s.db.group_is_used(p.generation_id).await?;
    tracing::debug!("Generation is used: {}", gen_group_used);
    if !gen_group_used {
	s.db.write_reply(p.generation_id, p.post_id).await?;
        let generated_reply = s.db.get_content_by_generation_id(p.generation_id).await?;
        Ok(CreateReplyButtonTemplate {
            post_id: p.post_id,
            generated_reply: Some(generated_reply),
        })
    } else {
        Err(WebError::GenerationAlreadyUsed)
    }
}
