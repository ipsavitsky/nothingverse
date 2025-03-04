use askama::Template;
use axum::extract::State;

use crate::AppState;

#[derive(Default)]
pub struct Post {
    id: i64,
    content: String,
    replies: Vec<String>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    new_posts: Vec<Post>,
    after_id: i64,
}

pub async fn handle(State(s): State<AppState>) -> IndexTemplate {
    let posts: Vec<Post> = futures::future::join_all(
        sqlx::query!(
            r#"SELECT
             id,
             content
           FROM posts
           ORDER BY timestamp DESC LIMIT 10"#
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

    IndexTemplate {
        after_id: posts.first().unwrap_or(&Post::default()).id,
        new_posts: posts,
    }
}
