use askama_axum::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomePage {
    page: String
}

pub async fn page_home() -> HomePage {
    HomePage {page: "home".to_string()}
}