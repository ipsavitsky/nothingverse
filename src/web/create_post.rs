use askama::Template;

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    index: u32,
}

pub async fn handle() -> NewPostTemplate {
    NewPostTemplate {
        index: 3,
    }
}
