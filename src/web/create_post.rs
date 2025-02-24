use askama::Template;

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    indices: Vec<u32>,
}

pub async fn handle() -> NewPostTemplate {
    NewPostTemplate {
        indices: vec![1, 2, 3],
    }
}
