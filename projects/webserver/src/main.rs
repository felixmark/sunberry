use std::{path::PathBuf, ptr::null, sync::Mutex};
use lazy_static::lazy_static;

use axum::{
    http::StatusCode,
    routing::get,
    Router,
    Json
};
use rusqlite::Connection;
use tower_http::{
    services::ServeDir, 
    services::ServeFile
};
use shared::dbstructs::{self, SystemMeasurement};
use tracing::info;
use serde::Serialize;
use chrono::{DateTime, Duration, Utc};

mod pages;

lazy_static! {
    static ref DB_CONNECTION: Mutex<Connection> = Mutex::new(Connection::open(PathBuf::from("/etc/sunberry/database.db")).expect("Could not establish DB connection."));
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Eeeeee?! (Error 404)")
}


#[derive(Serialize)]
struct JsonResponse<T> {
    data: T
}

async fn get_ina_db_entries(table_name: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Json<JsonResponse<Vec<dbstructs::INAMeasurement>>>, (StatusCode, String)> {
    let db_connection = DB_CONNECTION.lock().unwrap();
    let string_statement = format!(
        "SELECT id, timestamp, current, voltage, power FROM {} WHERE timestamp >= \"{}\" AND timestamp <= \"{}\"",
        table_name, from, to
    );
    let mut stmt = db_connection.prepare(&string_statement).expect("Selecting did not work.");
    let entry_iter = stmt.query_map([], |row| {
        Ok(dbstructs::INAMeasurement {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            current: row.get(2)?,
            voltage: row.get(3)?,
            power: row.get(4)?,
        })
    }).expect("Could not get measurements.");

    let entries = entry_iter.flatten().collect();
    Ok(Json(JsonResponse{data: entries}))
}

async fn get_power_consumption() -> Result<Json<JsonResponse<Vec<dbstructs::INAMeasurement>>>, (StatusCode, String)> {
    // TODO Get from and to from request parameters
    let from = Utc::now() - Duration::days(1);
    let to = Utc::now();
    get_ina_db_entries("power_consumptions", from, to).await
}

async fn get_power_pv() -> Result<Json<JsonResponse<Vec<dbstructs::INAMeasurement>>>, (StatusCode, String)> {
    // TODO Implement pv_power table and such
    let from = Utc::now() - Duration::days(1);
    let to = Utc::now();
    get_ina_db_entries("pv_power", from, to).await
}

async fn get_system_info_data() -> Result<Json<JsonResponse<Vec<SystemMeasurement>>>, (StatusCode, String)> {
    let db_connection = DB_CONNECTION.lock().unwrap();
    let mut stmt = db_connection.prepare(
        "SELECT 
            id,
            timestamp,
            used_memory_percent,
            used_swap_percent,
            used_disk_percent,
            used_cpu_percent,
            cpu_temperature,
            running_processes
            FROM system_logs"
        ).expect("Selecting did not work.");
    let entry_iter = stmt.query_map([], |row| {
        Ok(dbstructs::SystemMeasurement {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            used_memory_percent: row.get(2)?,
            used_swap_percent: row.get(3)?,
            used_disk_percent: row.get(4)?,
            used_cpu_percent: row.get(5)?,
            cpu_temperature: row.get(6)?,
            running_processes: row.get(7)?,
        })
    }).expect("Could not get measurements.");

    // Flatten equivalent: filter(|e| e.is_ok()).map(|e| e.unwrap())
    let entries = entry_iter.flatten().collect();
    Ok(Json(JsonResponse{data: entries}))
}

#[tracing::instrument(ret)]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    info!("{}", shared::predef::separator());
    info!("Webserver started");
    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("static"));
    let app = Router::new()
        .route("/", get(pages::home::page_home))
        .route("/systeminfo", get(pages::systeminfo::page_systeminfo))
        .route("/book", get(pages::mdpage::page_book))
        .route("/api/v1/power_pv", get(get_power_pv))
        .route("/api/v1/power_consumption", get(get_power_consumption))
        .route("/api/v1/system", get(get_system_info_data))
        .nest_service("/static", serve_dir)
        .fallback(fallback);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap();
}
