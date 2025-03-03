use askama::Template;
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use futures::stream::{self, Stream};
use ollama_rs::{error::OllamaError, generation::completion::request::GenerationRequest, Ollama};
use std::time::Duration;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::AppState;

#[derive(Template)]
#[template(path = "post_button.html")]
struct PostButton {
    index: String,
    post_data: String,
}

pub async fn handle(
    State(s): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, OllamaError>>> {
    let ollama = Ollama::new(s.conf.ollama_url, s.conf.ollama_port);
    let mut res = String::new();
    let stream = ollama
        .generate_stream(GenerationRequest::new(s.conf.model, s.conf.prompt))
        .await
        .unwrap()
        .map(move |x| {
            for resp in x? {
                res += resp.response.as_str();
                if resp.done {
                    res = PostButton {
                        index: Uuid::new_v4().to_string(),
                        post_data: res.clone(),
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
