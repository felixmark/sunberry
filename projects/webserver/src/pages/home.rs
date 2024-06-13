use askama_axum::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomePage;

pub async fn page_home() -> HomePage {
    HomePage
}