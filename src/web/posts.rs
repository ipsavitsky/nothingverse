use askama::Template;
use axum::extract::State;

use crate::AppState;

pub struct Post {
    content: String,
}

#[derive(Template)]
#[template(path="posts.html")]
pub struct PostsTemplate{
    posts: Vec<Post>,
}

pub async fn handle(State(s): State<AppState>) -> PostsTemplate {
    let posts = sqlx::query_as!(Post, "SELECT content FROM posts ORDER BY timestamp DESC LIMIT 10")
        .fetch_all(&s.db_pool)
        .await
        .unwrap();
    PostsTemplate { posts }
}
