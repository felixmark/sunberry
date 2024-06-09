use actix_web::{web, Responder, Result};
use actix_web::error::ErrorNotFound;
use actix_web_lab::respond::Html;
use askama::Template;
use tokio;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "mdpage.html")]
struct MarkdownPage<'a> {
    title: &'a str,
    md_content: &'a str,
}

pub async fn index() -> Result<impl Responder> {
    deliver_md_file("home".to_string()).await
}

pub async fn subpage(path: web::Path<String>) -> Result<impl Responder> {
    deliver_md_file(path.to_string()).await
}

async fn deliver_md_file(md_file: String) -> Result<impl Responder> {
    // Build markdown filepath
    let lowercase_md_file = md_file.to_lowercase();
    let mut md_file_path = PathBuf::new();
    md_file_path.push("markdown");
    md_file_path.push(lowercase_md_file);
    md_file_path.set_extension("md");

    // If markdown file does NOT exist, return 404 page
    let Ok(md_content) = tokio::fs::read_to_string(md_file_path).await else {
        return Err(ErrorNotFound("Requested page does not exist."));
    };
    
    // Markdown file exists. Deliver it.
    let html = MarkdownPage{
        title: &md_file.replace("_", " "),
        md_content: &md_content
    }.render().expect("Template should be valid");
    return Ok(Html(html));
}