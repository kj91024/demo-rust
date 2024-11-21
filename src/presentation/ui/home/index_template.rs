use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct IndexTemplate<'a> {
    pub name: String,
    pub age: u32,
    pub items: Vec<&'a str>
}