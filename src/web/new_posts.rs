use askama::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::AppState;

#[derive(Default)]
pub struct Post {
    id: i64,
    content: String,
}

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
    let new_posts = sqlx::query_as!(
        Post,
        "SELECT id, content FROM posts WHERE id > ? ORDER BY timestamp DESC",
        form.after
    )
    .fetch_all(&s.db_pool)
    .await
    .unwrap();
    PostsTemplate {
        after_id: new_posts
            .first()
            .unwrap_or(&Post {
                id: form.after,
                content: String::new(),
            })
            .id,
        new_posts,
    }
}
