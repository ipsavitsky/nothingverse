use askama::Template;

use axum::extract::State;

use crate::AppState;

pub struct Post {
    content: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    posts: Vec<Post>,
}

pub async fn handle(State(s): State<AppState>) -> IndexTemplate {
    let posts = sqlx::query_as!(Post, "SELECT content FROM posts")
        .fetch_all(&s.db_pool)
        .await
        .unwrap();
    IndexTemplate { posts }
}
