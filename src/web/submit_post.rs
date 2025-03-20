use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::AppState;

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "create_post_button.html")]
pub struct CreatePostButtonTemplate {}

#[derive(Deserialize)]
pub struct PostData {
    generation_id: i64,
}

pub async fn handle(
    State(s): State<AppState>,
    Form(f): Form<PostData>,
) -> Result<CreatePostButtonTemplate, WebError> {
    tracing::info!("Creating new post: {}", f.generation_id);
    s.db.write_post(f.generation_id).await?;
    Ok(CreatePostButtonTemplate {})
}
