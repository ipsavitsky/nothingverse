use askama::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::AppState;

#[derive(Default)]
pub struct Post {
    id: i64,
    content: String,
    replies: Vec<String>,
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
    let posts: Vec<Post> = futures::future::join_all(
        sqlx::query!(
            r#"SELECT
             id,
             content
           FROM posts
           WHERE posts.id > ?
           ORDER BY posts.timestamp DESC"#,
            form.after
        )
        .fetch_all(&s.db_pool)
        .await
        .unwrap()
        .into_iter()
        .map(async |r| Post {
            id: r.id,
            content: r.content,
            replies: sqlx::query!("SELECT content FROM replies WHERE post_id = ?", r.id)
                .fetch_all(&s.db_pool)
                .await
                .unwrap()
                .into_iter()
                .map(|r| r.content)
                .collect(),
        }),
    )
    .await;

    PostsTemplate {
        after_id: posts
            .first()
            .unwrap_or(&Post {
                id: form.after,
                content: String::new(),
                replies: Vec::default(),
            })
            .id,
        new_posts: posts,
    }
}
