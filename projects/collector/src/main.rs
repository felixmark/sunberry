/*
Collector periodically collects data from two INA 226 sensors over i2c 
and stores it in an SQLite Database.
*/

use rusqlite::{Connection, Result};
use std::path::PathBuf;
use chrono::Utc;
use chrono::naive::NaiveDateTime; 

#[derive(Debug)]
struct INAMeasurement {
    // Unsigned 128 bit (max 18,446,744,073,709,551,615 entries)
    id: u64, // ignore: unused
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
    create_table_power_consumption(&conn)
}

fn insert_measurement_into_power_consumption(conn: &Connection, measurement: INAMeasurement) -> Result<usize, rusqlite::Error> {
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

fn main() -> Result<()> {
    let mut db_filepath = PathBuf::new();
    db_filepath.push("/etc");
    db_filepath.push("sunberry");
    db_filepath.push("database");
    db_filepath.set_extension("db");
    let conn = Connection::open(db_filepath)?;

    create_tables(&conn)?;
    let my_measurement = INAMeasurement {
        id: 0,
        timestamp: Utc::now().naive_utc(),
        current: 0.01,
        voltage: 18.0,
        power: 0.18
    };
    insert_measurement_into_power_consumption(&conn, my_measurement)?;

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
}