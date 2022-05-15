use askama::Template;

#[derive(Template)]
#[template(path="client/index.html")]
pub struct Index {}
