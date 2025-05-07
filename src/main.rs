use axum::{
    Router,
    http::header,
    routing::{get, post},
};
use clap::Parser;
use sqlx::{Sqlite, migrate::MigrateDatabase, sqlite::SqlitePoolOptions};
use url::Url;
mod state_db;
mod web;

#[derive(Clone)]
struct AppState {
    db: state_db::StateDB,
    conf: Conf,
}

#[derive(Parser, Clone)]
struct Conf {
    /// App url
    #[arg(long, default_value_t = Url::parse("http://localhost:5000").unwrap())]
    url: Url,
    /// Ollama URL
    #[arg(long, default_value_t = Url::parse("http://localhost:11434").unwrap())]
    ollama_url: Url,
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

    if !Sqlite::database_exists(&conf.db_url).await.unwrap() {
        tracing::warn!("database file {} not found, creating...", &conf.db_url);
        Sqlite::create_database(&conf.db_url).await.unwrap();
    }

    let db_pool = SqlitePoolOptions::new()
        .connect(&conf.db_url)
        .await
        .expect("Could not create connection pool");

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Could not run migrations");

    let app = Router::new()
        .route("/", get(web::index::handle))
        .route("/create_post", post(web::create_post::handle))
        .route("/create_reply/{post_id}", post(web::create_reply::handle))
        .route(
            "/generate_post/{generation_group}",
            get(web::generate_post::handle),
        )
        .route(
            "/generate_reply/{post_id}/{generation_group}",
            get(web::generate_reply::handle),
        )
        .route(
            "/submit_post/{generation_id}",
            post(web::submit_post::handle),
        )
        .route(
            "/submit_reply/{post_id}/{generation_id}",
            post(web::submit_reply::handle),
        )
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
            conf: conf.clone(),
        });

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        conf.url.host().unwrap(),
        conf.url.port().unwrap()
    ))
    .await
    .expect("Could not bind address");

    tracing::info!("starting server");
    axum::serve(listener, app)
        .await
        .expect("Could now start server");
}
