use std::{path::PathBuf, sync::{Arc, Mutex}};
use axum::{
    routing::get,
    Router,
};
use rusqlite::Connection;
use shared::ezlogger::{EZLogger, ERROR_INITIALIZE};
use tower_http::{
    services::ServeDir, 
    services::ServeFile
};
use log::{debug, info, trace, warn, error, LevelFilter};

mod pages;

struct AppState {
    db_connection: Mutex<Connection>
}

#[tokio::main]
async fn main() {
    log::set_logger(Box::leak(Box::new(EZLogger::new("/var/log/sunberry/webserver.log")))).expect(ERROR_INITIALIZE);
    log::set_max_level(LevelFilter::Info);
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
        // API v1
        .nest("/api/v1", pages::apiv1::router())
        // Static files
        .nest_service("/static", serve_dir)
        // 404 fallback
        .fallback(pages::other::fallback)
        // Pass state into calls
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap();
}
