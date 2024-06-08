use std::collections::HashMap;
use actix_web::{web, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;
use tokio;

#[derive(Template)]
#[template(path = "notes.html")]
struct Notes<'a> {
    markdown: &'a str,
}

pub async fn page_notes(_query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let notes_md = tokio::fs::read_to_string("content/notes.md").await?;
    let notes_html = markdown::to_html(&notes_md);
    let html = Notes{
        markdown: &notes_html
    }.render().expect("Template should be valid");
    Ok(Html(html))
}
