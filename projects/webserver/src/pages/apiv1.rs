

use std::sync::Arc;
use serde::Serialize;
use chrono::{DateTime, Duration, Utc};
use axum::{
    extract::State, http::StatusCode, routing::get, Json, Router
};
use shared::dbstructs::{self, INAMeasurement, SystemMeasurement};
use crate::AppState;

#[derive(Serialize)]
pub struct JsonResponse<T> {
    data: T,
    timestamp: DateTime<Utc>
}

async fn get_ina_db_entries(State(state): State<Arc<AppState>>, table_name: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Json<JsonResponse<Vec<INAMeasurement>>>, (StatusCode, &'static str)> {
    let db_connection = state.db_connection.lock().unwrap();
    let string_statement = format!("SELECT 
        id, 
        min(timestamp), 
        avg(current), 
        avg(voltage), 
        avg(power) 
        FROM {} 
        WHERE timestamp >= \"{}\" AND timestamp <= \"{}\" 
        GROUP BY strftime('%s',timestamp) / 600",
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
    Ok(Json(JsonResponse{
        data: entries,
        timestamp: Utc::now()
    }))
}

pub async fn get_power_consumption(State(state): State<Arc<AppState>>) -> Result<Json<JsonResponse<Vec<INAMeasurement>>>, (StatusCode, &'static str)> {
    // TODO Get from and to from request parameters
    let from = Utc::now() - Duration::days(7);
    let to = Utc::now();
    get_ina_db_entries(State(state), "power_consumptions", from, to).await
}

pub async fn get_power_pv(State(state): State<Arc<AppState>>) -> Result<Json<JsonResponse<Vec<INAMeasurement>>>, (StatusCode, &'static str)> {
    // TODO Implement pv_power table and such
    let from = Utc::now() - Duration::days(7);
    let to = Utc::now();
    get_ina_db_entries(State(state), "pv_powers", from, to).await
}

pub async fn get_system_info_data(State(state): State<Arc<AppState>>) -> Result<Json<JsonResponse<Vec<SystemMeasurement>>>, (StatusCode, &'static str)> {
    let db_connection = state.db_connection.lock().unwrap();
    let from = Utc::now() - Duration::days(1);
    let to = Utc::now();

    let string_statement = format!("SELECT 
        id,
        min(timestamp),
        avg(used_memory_percent),
        avg(used_swap_percent),
        avg(used_disk_percent),
        avg(used_cpu_percent),
        avg(cpu_temperature),
        max(running_processes)
        FROM system_logs
        WHERE timestamp >= \"{}\" AND timestamp <= \"{}\"
        GROUP BY strftime('%s',timestamp) / 600",
        from, to);
    let mut stmt = db_connection.prepare(&string_statement).expect("Selecting did not work.");
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
    Ok(Json(JsonResponse{
        data: entries,
        timestamp: Utc::now()
    }))
}

pub fn router() -> axum::Router<std::sync::Arc<AppState>> {
    Router::new()
        .route("/power_pv", get(get_power_pv))
        .route("/power_consumption", get(get_power_consumption))
        .route("/system", get(get_system_info_data))
}
