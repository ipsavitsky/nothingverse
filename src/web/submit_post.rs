use askama::Template;

#[derive(Template)]
#[template(path = "create_post_button.html")]
pub struct CreatePostButtonTemplate {}

pub async fn handle(body: String) -> CreatePostButtonTemplate {
    tracing::info!("Creating new post: {}", body);
    CreatePostButtonTemplate {}
}
