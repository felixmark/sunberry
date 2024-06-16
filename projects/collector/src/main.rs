/*
Collector periodically collects data from two INA 226 sensors over i2c 
and stores it in an SQLite Database.
*/

use rusqlite::{Connection, Result};
use std::{path::PathBuf, time::{Duration, Instant}};
use chrono::Utc;
use std::{thread, time};
use log::{debug, error, info, trace, warn, LevelFilter};
use rand::Rng;
use sysinfo::{
    Components, Disks, System
};

use shared::{dbstructs::{self, SystemMeasurement}, ezlogger::{EZLogger, ERROR_INITIALIZE}};

static LOOP_INTERVAL_SECONDS: Duration = Duration::from_secs(5);

fn create_table_power_consumptions(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS power_consumptions (
            id        INTEGER PRIMARY KEY,
            timestamp TEXT,
            current   FLOAT,
            voltage   FLOAT,
            power     FLOAT
        )",
        (), // empty list of parameters.
    )
}

fn create_table_system_logs(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS system_logs (
            id                  INTEGER PRIMARY KEY,
            timestamp           TEXT,
            used_memory_percent FLOAT,
            used_swap_percent   FLOAT,
            used_disk_percent   FLOAT,
            used_cpu_percent    FLOAT,
            cpu_temperature     FLOAT,
            running_processes   INT
        )", ()
    )
}

fn create_tables(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut total_len = 0;
    total_len += create_table_power_consumptions(conn)?;
    total_len += create_table_system_logs(conn)?;
    Ok(total_len)
}

fn insert_measurement_into_power_consumption(conn: &Connection, measurement: dbstructs::INAMeasurement) -> Result<usize, rusqlite::Error> {
    debug!("Inserting measurement into database: {:?}", measurement);
    conn.execute(
        "INSERT INTO power_consumptions (timestamp, current, voltage, power) VALUES (?1, ?2, ?3, ?4)",
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
            timestamp,
            used_memory_percent,
            used_swap_percent,
            used_disk_percent,
            used_cpu_percent,
            cpu_temperature,
            running_processes
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

fn get_system_measurement() -> dbstructs::SystemMeasurement {
    let sys = System::new_all();

    // Disk usage
    let disks = Disks::new_with_refreshed_list();
    let mut largest_disk = 0;
    let mut largest_disk_usage = 0.0;
    for disk in disks.iter() {
        if disk.total_space() > largest_disk {
            largest_disk = disk.total_space();
            largest_disk_usage = (
                disk.total_space() as f32 - disk.available_space() as f32
                ) / disk.total_space() as f32;
        }
    }

    // CPU usage
    let mut average_cpu_usage = 0.0;
    for cpu in sys.cpus() {
        average_cpu_usage += cpu.cpu_usage();
    }
    average_cpu_usage /= sys.cpus().len() as f32;

    // CPU temp
    let mut cpu_temperature = 0.0;
    for component in Components::new_with_refreshed_list().iter() {
        // There is only one component in RasPi Zero 2 so this is fine
        cpu_temperature = component.temperature();
    }

    SystemMeasurement {
        id: 0,
        timestamp: Utc::now().naive_utc(),
        used_memory_percent: (sys.available_memory() as f32 / sys.total_memory() as f32) * 100.0,
        used_swap_percent: (sys.used_swap() as f32 / sys.total_swap() as f32) * 100.0,
        running_processes: sys.processes().len() as i32,
        used_disk_percent: largest_disk_usage,
        used_cpu_percent: average_cpu_usage,
        cpu_temperature
    }
}

fn collect(conn: &Connection) {
    debug!("Collecting souls.");

    // System measurements
    let system_measurement = get_system_measurement();
    let res = insert_measurement_into_system(conn, system_measurement);
    if res.is_err() {
        error!("Unable to insert data into the database: {:?}", res.err());
    }

    // INA measurement will be here
    let mut rng = rand::thread_rng();
    let current = rng.gen_range(0.0..1.0);
    let voltage = rng.gen_range(0.0..10.0);
    let fake_ina_226_measurement = dbstructs::INAMeasurement {
        id: 0,  // Will be overwritten
        timestamp: Utc::now().naive_utc(),
        current,
        voltage,
        power: current * voltage
    };
    let res = insert_measurement_into_power_consumption(conn, fake_ina_226_measurement);
    if res.is_err() {
        error!("Unable to insert data into the database.")
    }
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