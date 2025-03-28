use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::AppState;

use crate::state_db::models::Post;

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    posts: Vec<Post>,
    after_id: i64,
    before_id: i64,
    old_posts: Vec<Post>,
    new_posts: Vec<Post>,
    generated_reply: Option<String>,
}

pub async fn handle(State(s): State<AppState>) -> Result<IndexTemplate, WebError> {
    let posts = s.db.get_latest_posts().await?;

    Ok(IndexTemplate {
        after_id: posts.first().map(|x| x.id).unwrap_or(0),
        before_id: posts.last().map(|x| x.id).unwrap_or(0),
        posts,
        old_posts: Vec::new(),
        new_posts: Vec::new(),
        generated_reply: None,
    })
}
