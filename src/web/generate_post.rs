use askama::Template;
use askama_web::WebTemplate;
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

use crate::AppState;

#[derive(Template, WebTemplate)]
#[template(path = "post_button.html")]
struct PostButton {
    generation_id: i64,
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
                    let generation_id =
                        futures::executor::block_on(s.db.clone().write_generation(res.clone()));

                    res = PostButton {
                        generation_id,
                        post_data: res.clone(),
                    }
                    .render()
                    .unwrap()
                }
            }
            Ok(Event::default().data(&res).event("generation_chunk"))
        });

    let ending_event = stream::once(async { Ok(Event::default().data("").event("close")) });

    Sse::new(stream.chain(ending_event)).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("Keep-alive"),
    )
}
