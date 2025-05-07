use askama::Template;
use askama_web::WebTemplate;
use axum::{Form, extract::State};
use serde::Deserialize;

use crate::{AppState, state_db::models::Post};

use super::error::WebError;

#[derive(Template, WebTemplate)]
#[template(path = "new_posts.html")]
pub struct PostsTemplate {
    after_id: i64,
    new_posts: Vec<Post>,
    generated_reply: Option<String>,
}

#[derive(Deserialize)]
pub struct NewPostData {
    after: i64,
}

pub async fn handle(
    State(s): State<AppState>,
    Form(form): Form<NewPostData>,
) -> Result<PostsTemplate, WebError> {
    let posts = s.db.get_posts_after_id(form.after).await?;

    Ok(PostsTemplate {
        after_id: posts.first().map(|x| x.id).unwrap_or(form.after),
        new_posts: posts,
        generated_reply: None,
    })
}
