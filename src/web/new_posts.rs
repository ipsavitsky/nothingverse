use askama::Template;
use axum::{extract::State, Form};
use serde::Deserialize;

use crate::AppState;

#[derive(Default)]
pub struct PostData {
    id: i64,
    content: String,
    reply_content: Option<String>,
}

pub struct Post {
    id: i64,
    content: String,
    replies: Vec<String>,
}

impl Into<Post> for PostData {
    fn into(self) -> Post {
        Post {
            id: self.id,
            content: self.content,
            replies: self
                .reply_content
                .unwrap_or_default()
                .split("<SEPARATOR>")
                .map(String::from)
                .collect(),
        }
    }
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
    let posts = sqlx::query_as!(
        PostData,
        r#"SELECT
             posts.id,
             posts.content,
             group_concat(replies.content, "<SEPARATOR>") as reply_content
           FROM posts
           LEFT JOIN replies on posts.id = replies.post_id
           WHERE posts.id > ?
           GROUP BY posts.id
           ORDER BY posts.timestamp DESC"#,
        form.after
    )
    .fetch_all(&s.db_pool)
    .await
    .unwrap();
    PostsTemplate {
        after_id: posts
            .first()
            .unwrap_or(&PostData {
                id: form.after,
                content: String::new(),
                reply_content: None,
            })
            .id,
        new_posts: posts.into_iter().map(|x| x.into()).collect(),
    }
}
