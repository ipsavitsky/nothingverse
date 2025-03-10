use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    index: u32,
}

pub async fn handle() -> NewPostTemplate {
    NewPostTemplate { index: 3 }
}
