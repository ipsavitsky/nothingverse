use askama::Template;

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate {
    indices: Vec<u32>,
    swap_target: String,
    post_endpoint: String,
}

pub async fn handle() -> NewPostTemplate {
    NewPostTemplate {
        indices: vec![1, 2, 3],
        swap_target: String::from("#post_area"),
        post_endpoint: String::from("/submit_post"),
    }
}
