use std::collections::HashMap;
use actix_web::{web, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

pub async fn page_index(_query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let html = Index.render().expect("Template should be valid");
    Ok(Html(html))
}