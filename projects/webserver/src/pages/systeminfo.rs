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


fn fill_number_string(number: u64, length: u64, fill_character: char, left: bool) -> String {
    let mut ret_str = number.to_string();
    let number_len = number.to_string().len() as u64;
    if length > number_len {
        let to_fill = length - number_len;
        for _ in 0..to_fill {
            if left {
                ret_str = fill_character.to_string() + ret_str.as_str();
            } else {
                ret_str = ret_str + fill_character.to_string().as_str();
            }
        }
    }
    ret_str
}

fn bytes_to_string(mut bytes: u64) -> String {
    let mut rest = 0;
    let mut unit = "";
    if bytes / 1000 > 0 {
        rest = bytes % 1000;
        bytes = bytes / 1000;
        unit = "k";
        if bytes / 1000 > 0 {
            rest = bytes % 1000;
            bytes = bytes / 1000;
            unit = "M";
            if bytes / 1000 > 0 {
                rest = bytes % 1000;
                bytes = bytes / 1000;
                unit = "G"
            }
        }
    }
    return fill_number_string(bytes, 3, ' ', true) + "." + 
        fill_number_string(rest, 3, '0', false).to_string().as_str() + " " + unit + "B"
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
        should_continue |= process.memory() <= 0;
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

    let disks = Disks::new_with_refreshed_list();
    let mut disk_strings = vec![];
    for disk in disks.list() {
        disk_strings.push(format!("{:?}", disk));
    }

    let networks = Networks::new_with_refreshed_list();
    let mut network_strings = vec![];
    for network in &networks {
        network_strings.push(format!("{:?}", network));
    }

    let components = Components::new_with_refreshed_list();
    let mut component_strings = vec![];
    for component in &components {
        component_strings.push(format!("{:?}", component));
    }
    
    SystemInfo {
        total_memory: total_memory,
        used_memory: used_memory,
        used_memory_percent: used_memory_percent,
        total_swap: total_swap,
        used_swap: used_swap,

        system_name: system_name,
        kernel_version: kernel_version,
        os_version: os_version,
        host_name: host_name,

        number_cpus: number_cpus,
        processes: processes,
        disks: disk_strings,
        networks: network_strings,
        components: component_strings
    }
}