use axum::{
    http::StatusCode,
    routing::get,
    Router,
    Json
};
use tower_http::{
    services::ServeDir, 
    services::ServeFile
};
use serde::Serialize;

mod pages;
use general;

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Eeeeee?! (Error 404)")
}

#[derive(Serialize)]
struct DBData {
    power_used: Vec<i32>
}

async fn get_db_data() -> Json<DBData> {
    let db_data = DBData {
        power_used: vec![1, 2, 1, 2, 1, 2]
    };
    Json(db_data)
}

#[tracing::instrument(ret)]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("{}", general::separator());
    tracing::info!("Starting Webserver.");

    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static"));
    let app = Router::new()
        .route("/", get(pages::home::page_home))
        .route("/systeminfo", get(pages::systeminfo::page_systeminfo))
        .route("/book", get(pages::mdpage::page_book))
        .route("/api/v1/db_data", get(get_db_data))
        .nest_service("/static", serve_dir.clone())
        .fallback(fallback);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8010")
        .await
        .unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap();
}
