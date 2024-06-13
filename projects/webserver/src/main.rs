use axum::{
    http::StatusCode,
    routing::get,
    Router,
};
use tower_http::{
    services::ServeDir, 
    services::ServeFile
};

mod pages;
use general;

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Bober kurwa! (Error 404)")
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
        .nest_service("/static", serve_dir.clone())
        .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}