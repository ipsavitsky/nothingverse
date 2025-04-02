use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::AppState;

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    index: u32,
    gen_group: i64,
}

pub async fn handle(State(s): State<AppState>) -> Result<NewPostTemplate, WebError> {
    Ok(NewPostTemplate {
        index: 3,
        gen_group: s.db.get_new_generation_group().await?,
    })
}
