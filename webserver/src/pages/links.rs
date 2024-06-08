use std::collections::HashMap;
use actix_web::{web, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "links.html")]
struct Links;

pub async fn page_links(_query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let html = Links.render().expect("Template should be valid");
    Ok(Html(html))
}
