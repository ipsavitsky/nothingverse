use axum::{
    routing::{get, post},
    Router,
};
use tracing_subscriber;
use version_check;

mod web;

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
        .route("/", get(web::index::handle))
        .route("/create_post", post(web::create_post::handle))
        .route("/generate_post", get(web::generate_post::handle))
        .route("/submit_post", post(web::submit_post::handle));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    let triple = version_check::triple().unwrap();
    tracing::info!("Compiled with {}.{}.{}", triple.0, triple.1, triple.2);
    tracing::info!("starting server");
    axum::serve(listener, app).await.unwrap();
}
