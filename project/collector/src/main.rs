use rusqlite::{Connection, Result};
use std::path::PathBuf;

#[derive(Debug)]
struct INAMeasurement {
    id: i32,
    current: f64,
    voltage: f64,
    power: f64,
}

fn main() -> Result<()> {
    let mut db_filepath = PathBuf::new();
    db_filepath.push("/etc");
    db_filepath.push("sunberry");
    db_filepath.push("database");
    db_filepath.set_extension("db");
    let conn = Connection::open(db_filepath)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS pv_measurements (
            id    INTEGER PRIMARY KEY,
            current  DOUBLE,
            voltage  DOUBLE,
            power    DOUBLE
        )",
        (), // empty list of parameters.
    )?;
    let my_measurement = INAMeasurement {
        id: 0,
        current: 0.01,
        voltage: 18.0,
        power: 0.18
    };
    conn.execute(
        "INSERT INTO pv_measurements (current, voltage, power) VALUES (?1, ?2, ?3)",
        (&my_measurement.current, &my_measurement.voltage, &my_measurement.power),
    )?;

    let mut stmt = conn.prepare("SELECT id, current, voltage, power FROM pv_measurements")?;
    let measurement_iter = stmt.query_map([], |row| {
        Ok(INAMeasurement {
            id: row.get(0)?,
            current: row.get(1)?,
            voltage: row.get(2)?,
            power: row.get(3)?,
        })
    })?;

    for measurement in measurement_iter {
        println!("Found measurement {:?}", measurement.unwrap());
    }
    Ok(())
}