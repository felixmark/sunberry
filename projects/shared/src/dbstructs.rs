use chrono::naive::NaiveDateTime;

#[derive(Debug)]
pub struct INAMeasurement {
    // Unsigned 64 bit (max 18,446,744,073,709,551,616 entries)
    pub id: u64,
    // Please only ever use UTC (no +/- something) 
    // for everything and let frontend handle time
    pub timestamp: NaiveDateTime,
    // Current measurement in Ampere
    pub current: f32,
    // Voltage measurement in Volt
    pub voltage: f32,
    // Power in W (Current * Voltage (for easy db requests))
    pub power: f32,
}

#[derive(Debug)]
pub struct SystemMeasurement {
    pub id: u64,
    pub timestamp: NaiveDateTime,
    pub used_memory_percent: f32,
    pub used_swap_percent: f32,
    pub used_disk_percent: f32,
    pub used_cpu_percent: f32,
    pub cpu_temperature: f32,
    pub running_processes: i32,
}

