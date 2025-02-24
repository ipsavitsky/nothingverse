use futures::stream::{self, Stream};
use std::time::Duration;
use axum::response::{sse::{Event, KeepAlive}, Sse};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama, error::OllamaError};
use tokio_stream::StreamExt;

pub async fn handle() -> Sse<impl Stream<Item = Result<Event, OllamaError>>> {
    let model = "llama3.2:latest".to_string();
    let prompt =
        "Write a very short post on any theme you'd like. One sentence, no extra info. You may use hashtags".to_string();

    let ollama = Ollama::default();
    let mut res = String::new();
    let stream = ollama
        .generate_stream(GenerationRequest::new(model, prompt))
        .await
        .unwrap()
        .map(move |x| {
            for resp in x? {
                res += resp.response.as_str();
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
