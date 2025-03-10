use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Path;
use serde::Deserialize;

#[derive(Template, WebTemplate)]
#[template(path = "new_reply.html")]
pub struct NewReplyTemplate {
    index: u32,
    post_id: i64,
}

#[derive(Deserialize)]
pub struct PathData {
    post_id: i64,
}

pub async fn handle(Path(p): Path<PathData>) -> NewReplyTemplate {
    NewReplyTemplate {
        index: 3,
        post_id: p.post_id,
    }
}
