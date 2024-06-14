use log::{Record, Level, Metadata};
use chrono::Local;
use colored::Colorize;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

pub static ERROR_INITIALIZE: &str = "Something went wrong initializing EZLogger.";
pub static ERROR_LOG_LEVEL_UNKNOWN: &str = "Unknown log level.";
pub static ERROR_LOG_FILE_COULD_NOT_BE_OPENED: &str = "Unknown log level.";
pub static ERROR_WRITING_TO_FILE: &str = "Could not write to log file.";
static FORMAT_STRING: &str = "%Y-%m-%d %H:%M:%S.%3f";

pub struct EZLogger<'a> {
    pub name: &'a str
}

#[tokio::main]
async fn write_to_log(log_text: &String, name: &str) {
    _ = tokio::fs::create_dir("/var/log/sunberry").await;
    let mut file_path = PathBuf::new();
    file_path.push("/var");
    file_path.push("log");
    file_path.push("sunberry");
    file_path.push(name);
    file_path.set_extension("log");
    let mut log_file = tokio::fs::OpenOptions::new()
        .create(true).append(true).open(file_path)
        .await.expect(ERROR_LOG_FILE_COULD_NOT_BE_OPENED);
    log_file.write_all(log_text.to_string().as_bytes()).await.expect(ERROR_WRITING_TO_FILE);
}

impl log::Log for EZLogger<'_> {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = match record.level() {
                Level::Debug => "DBG".bold().to_string(),
                Level::Info => "INF".bold().green().to_string(),
                Level::Warn => "WRN".bold().yellow().to_string(),
                Level::Error => "ERR".bold().red().to_string(),
                _ => panic!("{}", ERROR_LOG_LEVEL_UNKNOWN),
            };

            let mut log_text = format!("{} {} {}",
                Local::now().format(FORMAT_STRING).to_string().black(),
                level_string, 
                record.args()
            );
            log_text.push('\n');
            print!("{}", &log_text);
            write_to_log(&log_text, &self.name);
        }
    }

    fn flush(&self) {}
}