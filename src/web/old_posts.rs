use askama::Template;
use askama_web::WebTemplate;
use axum::{Form, extract::State};
use serde::Deserialize;

use crate::{AppState, state_db::models::Post};

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "old_posts.html")]
pub struct OldPostsTemplate {
    before_id: i64,
    old_posts: Vec<Post>,
    generated_reply: Option<String>,
}

#[derive(Deserialize)]
pub struct OldPostData {
    before: i64,
}

pub async fn handle(
    State(s): State<AppState>,
    Form(form): Form<OldPostData>,
) -> Result<OldPostsTemplate, WebError> {
    let posts = s.db.get_posts_before_id(form.before).await?;

    Ok(OldPostsTemplate {
        before_id: posts.last().map(|x| x.id).unwrap_or(form.before),
        old_posts: posts,
        generated_reply: None,
    })
}
