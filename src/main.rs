use axum::{
    http::header,
    routing::{get, post},
    Router,
};
use clap::Parser;
use sqlx::sqlite::SqlitePoolOptions;

mod state_db;
mod web;

#[derive(Clone)]
struct AppState {
    db: state_db::StateDB,
    conf: Conf,
}

#[derive(Parser, Clone)]
struct Conf {
    /// Ollama URL
    #[arg(long, default_value_t = String::from("http://localhost"))]
    ollama_url: String,
    /// Ollama port
    #[arg(long, default_value_t = 11434)]
    ollama_port: u16,
    /// Language model to use from the ollama instance
    #[arg(short, long, default_value_t = String::from("nothing:latest"))]
    model: String,
    /// Prompt to create new posts and replies
    #[arg(short, long, default_value_t = String::from("Write a very short post on any theme you'd like. One sentence, no extra info. You should use hashtags. Do not quote your response or write any additional information, just the post"))]
    prompt: String,
    /// URL to database
    #[arg(long, default_value_t = String::from("sqlite://db/nothing.sqlite"))]
    db_url: String,
    /// Log level
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    log_level: tracing::Level,
}

#[tokio::main]
async fn main() {
    let conf = Conf::parse();

    tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_max_level(conf.log_level)
        .init();

    let db_pool = SqlitePoolOptions::new()
        .connect(&conf.db_url)
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Could not run migrations");

    let app = Router::new()
        .route("/", get(web::index::handle))
        .route("/create_post", post(web::create_post::handle))
        .route("/create_reply/{post_id}", post(web::create_reply::handle))
        .route("/generate_post", get(web::generate_post::handle))
        .route(
            "/generate_reply/{post_id}",
            get(web::generate_reply::handle),
        )
        .route("/submit_post", post(web::submit_post::handle))
        .route("/submit_reply/{post_id}", post(web::submit_reply::handle))
        .route("/new_posts", get(web::new_posts::handle))
        .route("/old_posts", get(web::old_posts::handle))
        .route_service(
            "/styles.css",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/css")],
                    include_str!("../templates/styles.css"),
                )
            }),
        )
        .route_service(
            "/htmx.min.js",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/javascript")],
                    include_str!("../templates/assets/htmx.min.js"),
                )
            }),
        )
        .route_service(
            "/sse.min.js",
            get(|| async {
                (
                    [(header::CONTENT_TYPE, "text/javascript")],
                    include_str!("../templates/assets/sse.min.js"),
                )
            }),
        )
        .with_state(AppState {
            db: state_db::StateDB { pool: db_pool },
            conf,
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .expect("Could not bind address");

    tracing::info!("starting server");
    axum::serve(listener, app)
        .await
        .expect("Could now start server");
}
