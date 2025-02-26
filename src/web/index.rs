use askama::Template;
use axum::extract::State;

use crate::AppState;

#[derive(Default)]
pub struct Post {
    id: i64,
    content: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    new_posts: Vec<Post>,
    after_id: i64,
}

pub async fn handle(State(s): State<AppState>) -> IndexTemplate {
    let posts = sqlx::query_as!(
        Post,
        "SELECT id, content FROM posts ORDER BY timestamp DESC LIMIT 10"
    )
    .fetch_all(&s.db_pool)
    .await
    .unwrap();

    IndexTemplate {
        after_id: posts.first().unwrap_or(&Post::default()).id,
        new_posts: posts,
    }
}
