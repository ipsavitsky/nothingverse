use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;

use crate::AppState;

use crate::state_db::models::Post;

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    new_posts: Vec<Post>,
    after_id: i64,
}

pub async fn handle(State(s): State<AppState>) -> Result<IndexTemplate, WebError> {
    let posts = s.db.get_latest_posts().await?;

    Ok(IndexTemplate {
        after_id: posts.first().map(|x| x.id).unwrap_or(0),
        new_posts: posts,
    })
}
