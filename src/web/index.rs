use askama::Template;
use axum::extract::State;

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
#[template(path = "index.html")]
pub struct IndexTemplate {
    new_posts: Vec<Post>,
    after_id: i64,
}

pub async fn handle(State(s): State<AppState>) -> IndexTemplate {
    let posts = sqlx::query_as!(
        PostData,
        r#"SELECT
             posts.id,
             posts.content,
             group_concat(replies.content, "<SEPARATOR>") AS reply_content
           FROM posts
           LEFT JOIN replies on posts.id = replies.post_id
           GROUP BY posts.id
           ORDER BY posts.timestamp DESC LIMIT 10"#
    )
    .fetch_all(&s.db_pool)
    .await
    .unwrap();

    IndexTemplate {
        after_id: posts.first().unwrap_or(&PostData::default()).id,
        new_posts: posts.into_iter().map(|x| x.into()).collect(),
    }
}
