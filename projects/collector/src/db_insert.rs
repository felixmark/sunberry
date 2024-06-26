use log::debug;
use rusqlite::Connection;
use shared::dbstructs;


pub fn insert_measurement_into(conn: &Connection, table_name: &str, measurement: dbstructs::INAMeasurement) -> Result<usize, rusqlite::Error> {
    let sql = format!("INSERT INTO {} (timestamp, current, voltage, power) VALUES (?1, ?2, ?3, ?4)", table_name);
    conn.execute(
        &sql.to_string(),
        (
            &measurement.timestamp.to_string(), 
            &measurement.current, 
            &measurement.voltage, 
            &measurement.power
        ),
    )
}

pub fn insert_measurement_into_system(conn: &Connection, measurement: dbstructs::SystemMeasurement) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO system_logs (
            timestamp, used_memory_percent, used_swap_percent, used_disk_percent, used_cpu_percent, cpu_temperature, running_processes
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            &measurement.timestamp.to_string(),
            &measurement.used_memory_percent,
            &measurement.used_swap_percent,
            &measurement.used_disk_percent,
            &measurement.used_cpu_percent,
            &measurement.cpu_temperature,
            &measurement.running_processes
        ),
    )
}