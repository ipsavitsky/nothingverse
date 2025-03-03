use askama::Template;
use axum::{
    extract::{Path, State},
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use futures::stream::{self, Stream};
use ollama_rs::{error::OllamaError, generation::completion::request::GenerationRequest, Ollama};
use serde::Deserialize;
use std::time::Duration;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::AppState;

#[derive(Template)]
#[template(path = "reply_button.html")]
struct ReplyButton {
    index: String,
    reply_data: String,
    post_id: i64,
}

#[derive(Deserialize)]
pub struct PathData {
    post_id: i64,
}

struct PostData {
    content: String,
}

pub async fn handle(
    Path(p): Path<PathData>,
    State(s): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, OllamaError>>> {
    let post = sqlx::query_as!(
        PostData,
        "SELECT content FROM posts where id = ?",
        p.post_id
    )
    .fetch_one(&s.db_pool)
    .await
    .unwrap();
    let ollama = Ollama::new(s.conf.ollama_url, s.conf.ollama_port);
    let mut res = String::new();
    let stream = ollama
        .generate_stream(GenerationRequest::new(
            s.conf.model,
            format!("Write a good reply to the following post: {}", post.content),
        ))
        .await
        .unwrap()
        .map(move |x| {
            for resp in x? {
                res += resp.response.as_str();
                if resp.done {
                    res = ReplyButton {
                        index: Uuid::new_v4().to_string(),
                        reply_data: res.clone(),
                        post_id: p.post_id,
                    }
                    .render()
                    .unwrap()
                }
            }
            Ok(Event::default().data(&res).event("generation_chunk"))
        });

    // This is needed so that the sse stream never gets dropped. HTMX is desinged to reconnect upon dropped stream to maintain consistency, but that is not what we want
    let infinite_stream =
        stream::repeat_with(|| Ok(Event::default())).throttle(Duration::from_secs(60));

    Sse::new(stream.merge(infinite_stream)).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("Keep-alive"),
    )
}
