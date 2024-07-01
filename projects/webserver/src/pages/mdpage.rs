use askama_axum::Template;
use axum::http::StatusCode;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "mdpage.html")]
pub struct MarkdownPage {
    page: String,
    md_content: String,
}

pub async fn page_book() -> Result<MarkdownPage, StatusCode> {
    deliver_md_file("book").await
}

async fn deliver_md_file(md_file: &str) -> Result<MarkdownPage, StatusCode> {
    // Build markdown filepath
    let lowercase_md_file = md_file.to_lowercase();
    let mut md_file_path = PathBuf::new();
    md_file_path.push("markdown");
    md_file_path.push(lowercase_md_file);
    md_file_path.set_extension("md");

    let Ok(md_content) = tokio::fs::read_to_string(&md_file_path).await else {
        return Err(StatusCode::NOT_FOUND);
    };

    // Markdown file exists. Deliver it.
    Ok(MarkdownPage {
        page: md_file.to_string(),
        md_content
    })
}