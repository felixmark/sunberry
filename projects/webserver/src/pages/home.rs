use actix_web::{Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct HomePage;

pub async fn home() -> Result<impl Responder> {
    let html = HomePage.render().expect("Template should be valid.");
    return Ok(Html(html));
}