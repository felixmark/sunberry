use askama_axum::Template;
use sysinfo::{
    Components, Disks, Networks, Pid, Process, System
};

struct ProcessInformation {
    name: String,
    memory: String,
    cpu_usage: String
}

#[derive(Template)]
#[template(path = "systeminfo.html")]
pub struct SystemInfo {
    total_memory: String,
    used_memory: String,
    used_memory_percent: String,
    total_swap: String,
    used_swap: String,
    system_name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,

    number_cpus: usize,
    processes: Vec<ProcessInformation>,
    disks: Vec<String>,
    networks: Vec<String>,
    components: Vec<String>
}

fn bytes_to_string(bytes: u64) -> String {
    match bytes {
        0..=999 => format!("{:<7} B", bytes),
        1_000..=999_999 => format!("{:<7.3} kB", bytes as f32 / 1_000f32),
        1_000_000..=999_999_999 => format!("{:<7.3} MB", bytes as f32 / 1_000_000f32),
        1_000_000_000.. => format!("{:<7.3} GB", bytes as f32 / 1_000_000_000f32),
    }
}

pub async fn page_systeminfo() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_memory = bytes_to_string(sys.total_memory());
    let used_memory = bytes_to_string(sys.used_memory());
    let used_memory_percent = (((sys.used_memory() as f64 / sys.total_memory() as f64) * 1000.0).round() / 10.0).to_string() + " %";
    let total_swap = bytes_to_string(sys.total_swap());
    let used_swap = bytes_to_string(sys.used_swap());

    let system_name = System::name().unwrap();
    let kernel_version = System::kernel_version().unwrap();
    let os_version = System::os_version().unwrap();
    let host_name = System::host_name().unwrap();
  
    let number_cpus = sys.cpus().len();
    let mut processes = vec![];
    let mut sys_processes: Vec<(&Pid, &Process)> = sys.processes().iter().collect();
    sys_processes.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));
    for (_pid, process) in sys_processes {
        let mut should_continue = process.name().starts_with("kworker");
        should_continue |= process.memory() == 0;
        if should_continue {
            continue;
        }
        let process_information = ProcessInformation {
            name: process.name().to_string(),
            memory: bytes_to_string(process.memory()),
            cpu_usage: format!("{:.5}", process.cpu_usage())
        };
        processes.push(process_information);
    }

    let disks: Vec<String> = Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| format!("{disk:?}"))
        .collect();
    
    let networks: Vec<String> = Networks::new_with_refreshed_list()
        .iter()
        .map(|network| format!("{network:?}"))
        .collect();
    
    let components: Vec<String> = Networks::new_with_refreshed_list()
        .iter()
        .map(|component| format!("{component:?}"))
        .collect();

    SystemInfo {
        total_memory,
        used_memory,
        used_memory_percent,
        total_swap,
        used_swap,

        system_name,
        kernel_version,
        os_version,
        host_name,

        number_cpus,
        processes,
        disks,
        networks,
        components
    }
}