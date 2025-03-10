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

use crate::AppState;

#[derive(Template)]
#[template(path = "reply_button.html")]
struct ReplyButton {
    generation_id: i64,
    reply_data: String,
    post_id: i64,
}

#[derive(Deserialize)]
pub struct PathData {
    post_id: i64,
}

pub async fn handle(
    Path(p): Path<PathData>,
    State(s): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, OllamaError>>> {
    let post_content = s.db.get_content_by_post_id(p.post_id).await;

    let ollama = Ollama::new(s.conf.ollama_url, s.conf.ollama_port);
    let mut res = String::new();
    let stream = ollama
        .generate_stream(GenerationRequest::new(
            s.conf.model,
            format!("Write a good reply to the following post: {}", post_content),
        ))
        .await
        .unwrap()
        .map(move |x| {
            for resp in x? {
                res += resp.response.as_str();
                if resp.done {
                    let generation_id =
                        futures::executor::block_on(s.db.clone().write_generation(res.clone()));

                    res = ReplyButton {
                        generation_id,
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
