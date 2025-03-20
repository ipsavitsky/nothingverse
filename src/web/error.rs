use crate::state_db::DBError;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use ollama_rs::error::OllamaError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("Error generating response: {0}")]
    Generation(#[from] OllamaError),
    #[error("Error querying database: {0}")]
    DB(#[from] DBError),
}

impl IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal error occured",
        )
            .into_response()
    }
}

#[derive(Error, Debug)]
pub enum GenerationError {
    #[error("Error getting generation chunk: {0}")]
    Ollama(#[from] OllamaError),
    #[error("Templating error")]
    Templating(#[from] askama::Error),
    #[error("Error querying database: {0}")]
    DB(#[from] DBError),
}
