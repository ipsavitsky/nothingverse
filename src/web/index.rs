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
             posts.id,
             generations.content
           FROM posts
           LEFT JOIN generations ON generations.id = posts.generation_id
           ORDER BY timestamp DESC LIMIT 10"#
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

    IndexTemplate {
        after_id: posts.first().unwrap_or(&Post::default()).id,
        new_posts: posts,
    }
}
