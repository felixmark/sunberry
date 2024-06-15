use log::{Record, Level, Metadata};
use chrono::Local;
use colored::Colorize;
use std::{io::{BufWriter, Write}, path::PathBuf};

pub static ERROR_INITIALIZE: &str = "Something went wrong initializing EZLogger.";
static ERROR_LOG_FILE_COULD_NOT_BE_OPENED: &str = "Failed to open log file.";
static ERROR_WRITING_TO_FILE: &str = "Could not write to log file.";
static ERROR_LOG_THREAD_CLOSED: &str = "Logging thread exited early.";

static FORMAT_STRING: &str = "%Y-%m-%d %H:%M:%S.%3f";

pub struct EZLogger {
    sender: std::sync::mpsc::SyncSender<String>,
}

impl EZLogger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let (sender, receiver) = std::sync::mpsc::sync_channel::<String>(1024);

        let path = path.into();
        std::thread::spawn(move || {
            let log_file = std::fs::OpenOptions::new()
                .create(true).append(true).open(path)
                .expect(ERROR_LOG_FILE_COULD_NOT_BE_OPENED);

            let mut writer = BufWriter::new(log_file);

            for message in receiver {
                print!("{}", &message);
                writer.write_all(message.as_bytes()).expect(ERROR_WRITING_TO_FILE);
                writer.flush().expect(ERROR_WRITING_TO_FILE);
            }
        });

        Self {
            sender
        }
    }
}

impl log::Log for EZLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_string = match record.level() {
                Level::Trace => "TRC".bold().black(),
                Level::Debug => "DBG".bold(),
                Level::Info => "INF".bold().green(),
                Level::Warn => "WRN".bold().yellow(),
                Level::Error => "ERR".bold().red(),
            };

            let log_text = format!("{} {} {}\n",
                Local::now().format(FORMAT_STRING).to_string().black(),
                level_string, 
                record.args()
            );

            self.sender.send(log_text).expect(ERROR_LOG_THREAD_CLOSED);
        }
    }

    fn flush(&self) {}
}