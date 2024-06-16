/*
Collector periodically collects data from two INA 226 sensors over i2c 
and stores it in an SQLite Database.
*/

use rusqlite::{Connection, Result};
use std::{path::PathBuf, time::{Duration, Instant}};
use std::{thread, time};
use log::{debug, error, info, trace, warn, LevelFilter};
use shared::{dbstructs::{self}, ezlogger::{EZLogger, ERROR_INITIALIZE}};

mod tables;
mod measure;

static LOOP_INTERVAL_SECONDS: Duration = Duration::from_secs(5);


fn create_tables(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut total_len = 0;
    total_len += tables::create_table_power_consumptions(conn)?;
    total_len += tables::create_table_system_logs(conn)?;
    total_len += tables::create_table_pv_power(conn)?;
    Ok(total_len)
}

fn insert_measurement_into(conn: &Connection, table_name: &str, measurement: dbstructs::INAMeasurement) -> Result<usize, rusqlite::Error> {
    debug!("Inserting measurement into {}: {:?}", table_name, measurement);
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

fn insert_measurement_into_system(conn: &Connection, measurement: dbstructs::SystemMeasurement) -> Result<usize, rusqlite::Error> {
    debug!("Inserting measurement into database: {:?}", measurement);
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

fn collect(conn: &Connection) -> Result<()> {
    debug!("Collecting souls.");

    // System measurements
    let system_measurement = measure::get_system_measurement();
    let res = insert_measurement_into_system(conn, system_measurement);
    if res.is_err() {
        error!("Unable to insert data into the database: {:?}", res.err());
    }
    insert_measurement_into(conn, "power_consumption", measure::get_power_usage_measurement())?;
    insert_measurement_into(conn, "pv_powers", measure::get_pv_power_measurement())?;
    Ok(())
}

fn main() -> Result<()> {
    log::set_logger(Box::leak(Box::new(EZLogger::new("/var/log/sunberry/collector.log")))).expect(ERROR_INITIALIZE);
    log::set_max_level(LevelFilter::Trace);

    info!("{}", shared::predef::separator());
    info!("Collector started");

    let db_filepath = PathBuf::from("/etc/sunberry/database.db");
    info!("Establishing connection to: {:?}", db_filepath);
    let conn = Connection::open(db_filepath)?;

    info!("Creating tables if they do not exist.");
    create_tables(&conn)?;

    trace!("Starting main loop.");
    let mut goal = time::Instant::now();
    loop {
        // Executed every LOOP_INTERVAL_SECONDS seconds
        collect(&conn);

        // Check loop time, calculate next loop time and sleep until then
        let now = Instant::now();
        goal += LOOP_INTERVAL_SECONDS;
        if goal < now {
            warn!("Collection running too slow!");
            while goal < now {
                goal += LOOP_INTERVAL_SECONDS;
            }
        }
        thread::sleep(goal - now);
    }
}