use askama_axum::Template;
use axum::http::StatusCode;
use markdown::{CompileOptions, Options, ParseOptions};
use std::{fs, io::ErrorKind, path::PathBuf};

#[derive(Template)]
#[template(path = "mdpage.html")]
pub struct MarkdownPage {
    page: String,
    md_content: String,
}

pub async fn page_fauna() -> Result<MarkdownPage, StatusCode> {
    deliver_md_file("fauna").await
}

pub async fn page_flora() -> Result<MarkdownPage, StatusCode> {
    deliver_md_file("flora").await
}

pub async fn page_funga() -> Result<MarkdownPage, StatusCode> {
    deliver_md_file("funga").await
}

pub async fn page_gestein() -> Result<MarkdownPage, StatusCode> {
    deliver_md_file("gestein").await
}

pub async fn page_wandersteine() -> Result<MarkdownPage, StatusCode> {
    deliver_md_file("wandersteine").await
}

pub async fn page_mein_wanderstein() -> Result<MarkdownPage, StatusCode> {
    deliver_md_file("mein-wanderstein").await
}

async fn deliver_md_file(md_file: &str) -> Result<MarkdownPage, StatusCode> {
    // Build markdown filepath
    let lowercase_md_file = md_file.to_lowercase();
    let mut md_file_path = PathBuf::new();
    md_file_path.push("markdown");
    md_file_path.push(lowercase_md_file);
    md_file_path.set_extension("md");

    let Ok(mut md_content) = tokio::fs::read_to_string(&md_file_path).await else {
        return Err(StatusCode::NOT_FOUND);
    };

    let folders = ["flora", "fauna", "funga", "gestein"];

    for folder in folders.into_iter() {
        let mut path = "/static/img/naturarchiv/".to_owned();
        path.push_str(folder);
        path.push('/');

        let mut full_path = ".".to_owned();
        full_path.push_str(&path);

        let paths: Vec<PathBuf> = match fs::read_dir(full_path) {
            Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),
            Err(e) => panic!("Unexpected Error! {:?}", e),
            Ok(entries) => entries.filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect()
        };
        let mut images = "".to_owned();
        for s in paths.into_iter() {
            let os_str_filename = s.file_name().expect("Could not unwrap file name from file.");
            let filename = os_str_filename.to_str().expect("Could not convert string path to string.");
            images.push_str("<img src='");
            images.push_str(&path);
            images.push_str(filename);
            images.push_str("'>");
        }

        let mut replacer = "{{".to_owned();
        replacer.push_str(folder);
        replacer.push_str("}}");

        md_content = md_content.replace(
            &replacer, 
            &images
        );
    }

    let parsed_md_content = markdown::to_html_with_options(&md_content, &Options {
        compile: CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..CompileOptions::default()
        },
        ..Options::default()
    }).expect("Could not parse markdown file!");

    // Markdown file exists. Deliver it.
    Ok(MarkdownPage {
        page: md_file.to_string(),
        md_content: parsed_md_content
    })
}