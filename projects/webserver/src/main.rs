use std::{path::PathBuf, sync::{Arc, Mutex}};

use axum::{
    http::StatusCode,
    routing::get,
    Router,
};
use rusqlite::Connection;
use tower_http::{
    services::ServeDir, 
    services::ServeFile
};
use tracing::info;

mod pages;

struct AppState {
    db_connection: Mutex<Connection>
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Eeeeee?! (Error 404)")
}

#[tracing::instrument(ret)]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    info!("{}", shared::predef::separator());
    info!("Webserver started");

    let shared_state = Arc::new(AppState {
        db_connection: Mutex::new(Connection::open(PathBuf::from("/etc/sunberry/database.db")).expect("Could not establish DB connection."))
    });
    
    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static"));
    let app = Router::new()
        .route("/", get(pages::home::page_home))
        .route("/systeminfo", get(pages::systeminfo::page_systeminfo))
        .route("/book", get(pages::mdpage::page_book))
        .route("/api/v1/power_pv", get(pages::api::get_power_pv))
        .route("/api/v1/power_consumption", get(pages::api::get_power_consumption))
        .route("/api/v1/system", get(pages::api::get_system_info_data))
        .nest_service("/static", serve_dir)
        .fallback(fallback)
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap();
}
