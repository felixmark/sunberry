use std::{env, path::PathBuf, string, sync::{Arc, Mutex}};
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
    let args: Vec<String> = env::args().collect();
    let mut port = 80;
    if args.len() > 1 && args[1] == "d" {
        port = 8080;
    }
    info!("{}", shared::predef::separator());
    info!("Webserver started");

    let shared_state = Arc::new(AppState {
        db_connection: Mutex::new(Connection::open(PathBuf::from("/etc/sunberry/database.db")).expect("Could not establish DB connection."))
    });
    
    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static"));
    let app = Router::new()
        .route("/", get(pages::home::page_home))
        .route("/fauna", get(pages::mdpage::page_fauna))
        .route("/flora", get(pages::mdpage::page_flora))
        .route("/funga", get(pages::mdpage::page_funga))
        .route("/gestein", get(pages::mdpage::page_gestein))
        .route("/wandersteine", get(pages::mdpage::page_wandersteine))
        .route("/mein-wanderstein", get(pages::mdpage::page_mein_wanderstein))
        .route("/system", get(pages::system::page_system))
        // API v1
        .nest("/api/v1", pages::apiv1::router())
        // Static files
        .nest_service("/static", serve_dir)
        // 404 fallback
        .fallback(pages::other::fallback)
        // Pass state into calls
        .with_state(shared_state);

    let address = "0.0.0.0:".to_owned() + &format!("{}", port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .unwrap();
    info!("Listening on {}", &address);
    axum::serve(listener, app)
        .await
        .unwrap();
}
