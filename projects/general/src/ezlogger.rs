use log::{Record, Level, Metadata};
use chrono::Local;
use colored::Colorize;

pub static INITIALIZE_ERROR: &str = "Something went wrong initializing EZLogger.";
pub static LOG_LEVEL_ERROR: &str = "Unknown log level.";
static FORMAT_STRING: &str = "%Y-%m-%d %H:%M:%S.%3f";

pub struct EZLogger;

impl log::Log for EZLogger {
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
                _ => panic!("{}", LOG_LEVEL_ERROR),
            };
            println!(
                "{} {} {}", 
                Local::now().format(FORMAT_STRING).to_string().black(),
                level_string, 
                record.args()
            );
        }
    }

    fn flush(&self) {}
}