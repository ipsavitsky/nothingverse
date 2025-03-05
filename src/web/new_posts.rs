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
             posts.id,
             generations.content
           FROM posts
           LEFT JOIN generations ON posts.generation_id = generations.id
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
            content: r.content.unwrap(),
            replies: sqlx::query!("SELECT generations.content FROM replies LEFT JOIN generations ON generations.id = replies.generation_id WHERE post_id = ?", r.id)
                .fetch_all(&s.db_pool)
                .await
                .unwrap()
                .into_iter()
                .map(|r| r.content.unwrap())
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
