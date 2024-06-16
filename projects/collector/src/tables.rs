use rusqlite::Connection;

pub fn create_tables(conn: &Connection) -> Result<usize, rusqlite::Error> {
    let mut total_len = 0;
    total_len += create_table_power_consumptions(conn)?;
    total_len += create_table_system_logs(conn)?;
    total_len += create_table_pv_power(conn)?;
    Ok(total_len)
}


pub fn create_table_power_consumptions(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS power_consumptions (
            id        INTEGER PRIMARY KEY,
            timestamp TEXT,
            current   FLOAT,
            voltage   FLOAT,
            power     FLOAT
        )", (), // empty list of parameters.
    )
}

pub fn create_table_pv_power(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pv_powers (
            id        INTEGER PRIMARY KEY,
            timestamp TEXT,
            current   FLOAT,
            voltage   FLOAT,
            power     FLOAT
        )", (),
    )
}

pub fn create_table_system_logs(conn: &Connection) -> Result<usize, rusqlite::Error> {
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