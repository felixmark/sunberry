use chrono::Utc;
use rand::Rng;
use shared::dbstructs::{self, INAMeasurement, SystemMeasurement};
use sysinfo::{
    Components, Disks, System
};


pub fn get_system_measurement() -> dbstructs::SystemMeasurement {
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
        used_memory_percent: (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0,
        used_swap_percent: (sys.used_swap() as f32 / sys.total_swap() as f32) * 100.0,
        running_processes: sys.processes().len() as i32,
        used_disk_percent: largest_disk_usage,
        used_cpu_percent: average_cpu_usage,
        cpu_temperature
    }
}

pub fn get_power_usage_measurement() -> INAMeasurement {
    // TODO Replace fake data with real data
    let mut rng = rand::thread_rng();
    let current = rng.gen_range(0.0..1.0);
    let voltage = rng.gen_range(0.0..10.0);
    dbstructs::INAMeasurement {
        id: 0,  // Will be overwritten
        timestamp: Utc::now().naive_utc(),
        current: rng.gen_range(0.0..1.0),
        voltage,
        power: current * voltage
    }
}

pub fn get_pv_power_measurement() -> INAMeasurement {
    // TODO Replace fake data with real data
    let mut rng = rand::thread_rng();
    let current = rng.gen_range(0.0..1.0);
    let voltage = rng.gen_range(0.0..10.0);
    dbstructs::INAMeasurement {
        id: 0,  // Will be overwritten
        timestamp: Utc::now().naive_utc(),
        current: rng.gen_range(0.0..1.0),
        voltage,
        power: current * voltage
    }
}
