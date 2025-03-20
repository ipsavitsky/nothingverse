use askama::Template;
use askama_web::WebTemplate;
use axum::{
    extract::{Path, State},
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use futures::stream::{self, Stream};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use serde::Deserialize;
use std::time::Duration;
use tokio_stream::StreamExt;

use crate::AppState;

use super::error::{GenerationError, WebError};

#[derive(Template, WebTemplate)]
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
) -> Result<Sse<impl Stream<Item = Result<Event, GenerationError>>>, WebError> {
    let post_content = s.db.get_content_by_post_id(p.post_id).await?;

    let ollama = Ollama::new(s.conf.ollama_url, s.conf.ollama_port);
    let mut res = String::new();
    let stream = ollama
        .generate_stream(GenerationRequest::new(
            s.conf.model,
            format!("Write a good reply to the following post: {}", post_content),
        ))
        .await?
        .map(move |x| {
            for resp in x? {
                res += resp.response.as_str();
                if resp.done {
                    let generation_id =
                        futures::executor::block_on(s.db.clone().write_generation(res.clone()))?;

                    res = ReplyButton {
                        generation_id,
                        reply_data: res.clone(),
                        post_id: p.post_id,
                    }
                    .render()?
                }
            }
            Ok(Event::default().data(&res).event("generation_chunk"))
        });

    let ending_event = stream::once(async { Ok(Event::default().data("").event("close")) });

    Ok(Sse::new(stream.chain(ending_event)).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("Keep-alive"),
    ))
}
