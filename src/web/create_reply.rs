use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use serde::Deserialize;

use crate::AppState;

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "new_reply.html")]
pub struct NewReplyTemplate {
    index: u32,
    post_id: i64,
    gen_group: i64,
}

#[derive(Deserialize)]
pub struct PathData {
    post_id: i64,
}

pub async fn handle(
    State(s): State<AppState>,
    Path(p): Path<PathData>,
) -> Result<NewReplyTemplate, WebError> {
    Ok(NewReplyTemplate {
        index: 3,
        post_id: p.post_id,
        gen_group: s.db.get_new_generation_group().await?,
    })
}
