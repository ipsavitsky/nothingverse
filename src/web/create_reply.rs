use askama::Template;
use axum::Form;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    indices: Vec<String>,
    swap_target: String,
    post_endpoint: String,
}

#[derive(Deserialize)]
pub struct ReplyData {
    post_id: String,
}

pub async fn handle(Form(f): Form<ReplyData>) -> NewPostTemplate {
    NewPostTemplate {
        indices: vec!["1", "2", "3"]
            .iter()
            .map(|s| f.post_id.clone() + s)
            .collect(),
        swap_target: format!("#reply_area_{}", f.post_id),
        post_endpoint: String::from("/submit_reply"),
    }
}
