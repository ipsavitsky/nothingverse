use askama::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::{state_db::models::Post, AppState};

#[derive(Template)]
#[template(path = "new_posts.html")]
pub struct PostsTemplate {
    after_id: i64,
    new_posts: Vec<Post>,
}

#[derive(Deserialize)]
pub struct NewPostData {
    after: i64,
}

pub async fn handle(State(s): State<AppState>, Form(form): Form<NewPostData>) -> PostsTemplate {
    let posts = s.db.get_posts_after_id(form.after).await;

    PostsTemplate {
        after_id: posts.first().map(|x| x.id).unwrap_or(form.after),
        new_posts: posts,
    }
}
