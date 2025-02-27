use askama::Template;

#[derive(Template)]
#[template(path = "new_reply.html")]
pub struct NewReplyTemplate {
    index: u32,
}

pub async fn handle() -> NewReplyTemplate {
    NewReplyTemplate {
        index: 3,
    }
}
