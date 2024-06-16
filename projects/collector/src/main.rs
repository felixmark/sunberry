/*
Collector periodically collects data from two INA 226 sensors over i2c 
and stores it in an SQLite Database.
*/

use rusqlite::{Connection, Result};
use std::{path::PathBuf, time::{Duration, Instant}};
use std::{thread, time};
use log::{debug, info, trace, warn, error, LevelFilter};
use shared::ezlogger::{EZLogger, ERROR_INITIALIZE};

mod tables;
mod measure;
mod db_insert;

static LOOP_INTERVAL_SECONDS: Duration = Duration::from_secs(5);


fn collect(conn: &Connection) -> Result<()> {
    debug!("Collecting souls.");
    db_insert::insert_measurement_into_system(conn, measure::get_system_measurement())?;
    db_insert::insert_measurement_into(conn, "power_consumptions", measure::get_power_usage_measurement())?;
    db_insert::insert_measurement_into(conn, "pv_powers", measure::get_pv_power_measurement())?;
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
    tables::create_tables(&conn)?;

    trace!("Starting main loop.");
    let mut goal = time::Instant::now();
    loop {
        // Executed every LOOP_INTERVAL_SECONDS seconds
        if collect(&conn).is_err() {
            error!("Unable to collect all the data!");
        }

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