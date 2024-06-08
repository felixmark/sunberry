use std::collections::HashMap;
use actix_web::{web, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;
use sysinfo::{
    Components, Disks, Networks, System,
};

#[derive(Template)]
#[template(path = "stats.html")]
struct Stats<'a> {
    total_memory: &'a str,
    used_memory: &'a str,
    used_memory_percent: &'a str,
    total_swap: &'a str,
    used_swap: &'a str,
    system_name: &'a str,
    kernel_version: &'a str,
    os_version: &'a str,
    host_name: &'a str,
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

pub async fn page_stats(_query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
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
  
    /*
    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
    
    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
    }
    
    // We display all disks' information:
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }
    
    // Network interfaces name, total data received and total data transmitted:
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
        // If you want the amount of data received/transmitted since last call
        // to `Networks::refresh`, use `received`/`transmitted`.
    }
    
    // Components temperature:
    let components = Components::new_with_refreshed_list();
    println!("=> components:");
    for component in &components {
        println!("{component:?}");
    }
    
    Please remember that to have some up-to-date information, you need to call the equivalent refresh method. For example, for the CPU usage:
    
    use sysinfo::System;
    
    let mut sys = System::new();
    
    loop {
        sys.refresh_cpu(); // Refreshing CPU information.
        for cpu in sys.cpus() {
            print!("{}% ", cpu.cpu_usage());
        }
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
    */

    
    let html = Stats {
        total_memory: &total_memory,
        used_memory: &used_memory,
        used_memory_percent: &used_memory_percent,
        total_swap: &total_swap,
        used_swap: &used_swap,

        system_name: &system_name,
        kernel_version: &kernel_version,
        os_version: &os_version,
        host_name: &host_name,
    }.render().expect("Template should be valid");
    Ok(Html(html))
}