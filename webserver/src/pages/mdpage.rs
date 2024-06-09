use actix_web::{web, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;
use tokio;

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
    let lowercase_md_file = md_file.to_lowercase();
    let mut filepath = "md/".to_owned();
    filepath.push_str(&lowercase_md_file);
    filepath.push_str(".md".to_owned().as_str());
    if tokio::fs::metadata(&filepath).await.is_ok() {
        let md_content = tokio::fs::read_to_string(filepath).await?;
        let html = MarkdownPage{
            title: &md_file,
            md_content: &md_content
        }.render().expect("Template should be valid");
        return Ok(Html(html));
    }
    return Ok(Html("Page not found, even though status code is 200...".to_string()))
}