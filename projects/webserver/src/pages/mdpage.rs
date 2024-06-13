use askama_axum::Template;
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "mdpage.html")]
pub struct MarkdownPage {
    title: String,
    md_content: String,
}

pub async fn page_book() -> MarkdownPage {
    deliver_md_file("book").await
}

async fn deliver_md_file(md_file: &str) -> MarkdownPage {
    // Build markdown filepath
    let lowercase_md_file = md_file.to_lowercase();
    let mut md_file_path = PathBuf::new();
    md_file_path.push("markdown");
    md_file_path.push(lowercase_md_file);
    md_file_path.set_extension("md");

    let Ok(md_content) = tokio::fs::read_to_string(&md_file_path).await else {
        panic!("Oh oh! {:?}", &md_file_path);
    };

    // Markdown file exists. Deliver it.
    MarkdownPage {
        title: md_file.replace("_", " "),
        md_content: md_content
    }
}