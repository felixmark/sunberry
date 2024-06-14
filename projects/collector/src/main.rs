/*
Collector periodically collects data from two INA 226 sensors over i2c 
and stores it in an SQLite Database.
*/

use rusqlite::{Connection, Result};
use std::{path::PathBuf, time::{Duration, Instant}};
use chrono::{
    Utc,
    naive::NaiveDateTime
};
use std::{thread, time};
use log::{debug, error, info, trace, warn, LevelFilter};
use rand::Rng;

use general::{ezlogger::EZLogger, ezlogger::ERROR_INITIALIZE};

static LOOP_INTERVAL_SECONDS: Duration = Duration::from_secs(5);

#[derive(Debug)]
struct INAMeasurement {
    // Unsigned 64 bit (max 18,446,744,073,709,551,616 entries)
    id: u64,
    // Please only ever use UTC (no +/- something) 
    // for everything and let frontend handle time
    timestamp: NaiveDateTime,
    // Current measurement in Ampere
    current: f32,
    // Voltage measurement in Volt
    voltage: f32,
    // Power in W (Current * Voltage (for easy db requests))
    power: f32,
}

fn create_table_power_consumption(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS power_consumption (
            id        INTEGER PRIMARY KEY,
            timestamp TEXT,
            current   FLOAT,
            voltage   FLOAT,
            power     FLOAT
        )",
        (), // empty list of parameters.
    )
}

fn create_tables(conn: &Connection) -> Result<usize, rusqlite::Error> {
    create_table_power_consumption(conn)
}

fn insert_measurement_into_power_consumption(conn: &Connection, measurement: INAMeasurement) -> Result<usize, rusqlite::Error> {
    debug!("Inserting measurement into database: {:?}", measurement);
    conn.execute(
        "INSERT INTO power_consumption (timestamp, current, voltage, power) VALUES (?1, ?2, ?3, ?4)",
        (
            &measurement.timestamp.to_string(), 
            &measurement.current, 
            &measurement.voltage, 
            &measurement.power
        ),
    )
}

fn collect(conn: &Connection) {
    debug!("Collecting souls.");
    let mut rng = rand::thread_rng();
    let current = rng.gen_range(0.0..1.0);
    let voltage = rng.gen_range(0.0..20.0);
    let fake_ina_226_measurement = INAMeasurement {
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

    info!("{}", general::predef::separator());
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
        // Measure time and run next loop in LOOP_INTERVAL_SECONDS - collect_time seconds
        // INFO: Drifts 0.001s every ~30s
        collect(&conn);

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
    
    /*
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, current, voltage, power FROM power_consumption"
    )?;
    let measurement_iter = stmt.query_map([], |row| {
        Ok(INAMeasurement {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            current: row.get(2)?,
            voltage: row.get(3)?,
            power: row.get(4)?,
        })
    })?;

    for measurement in measurement_iter {
        println!("Found {:?}", measurement.unwrap());
    }
    Ok(())
    */
}