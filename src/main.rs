use std::time::Duration;

use askama::Template;
use axum::{
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Router,
};
use futures::stream::Stream;
use ollama_rs::{error::OllamaError, generation::completion::request::GenerationRequest, Ollama};
use tokio_stream::StreamExt as _;
use tracing_subscriber;
use version_check;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {}

#[derive(Template)]
#[template(path = "new_post.html")]
struct NewPostTemplate {}

async fn home() -> HomeTemplate {
    HomeTemplate {}
}

async fn create_post() -> NewPostTemplate {
    NewPostTemplate {}
}

async fn generate_post() -> Sse<impl Stream<Item = Result<Event, OllamaError>>> {
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

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("Keep-alive"),
    )
}

async fn submit_post(body: String) {
    tracing::info!("Creating new post: {}", body);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_max_level(tracing::Level::TRACE)
        .init();

    let app = Router::new()
        .route("/", get(home))
        .route("/create_post", post(create_post))
        .route("/generate_post", get(generate_post))
        .route("/submit_post", post(submit_post));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    let triple = version_check::triple().unwrap();
    tracing::info!("Compiled with {}.{}.{}", triple.0, triple.1, triple.2);
    tracing::info!("starting server");
    axum::serve(listener, app).await.unwrap();
}
