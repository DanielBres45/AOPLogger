use std::sync::OnceLock;
use std::{fs::File, panic};

use chrono::Local;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

use crate::logger::log_message_parser::LogMessage;

use super::log_message_parser::LogMessageParser;
use super::panic_handler::PanicHandler;
use super::thread_buffer::ThreadSafeBufferedWriter;

pub struct LoggingManager {
    writer: ThreadSafeBufferedWriter<File>,
    level: Level,
}

impl Drop for LoggingManager {
    fn drop(&mut self) {
        self.flush();
    }
}

//Made static so that we can flush the log by calling LoggingManager::static_flush();
static LOGGER: OnceLock<&'static LoggingManager> = OnceLock::new();

impl LoggingManager {
    pub fn init(file_path: &str) -> Result<(), SetLoggerError> {
        let writer = match ThreadSafeBufferedWriter::new(file_path) {
            Ok(buf) => buf,
            Err(e) => panic!("Failed to create new buffered logger {}", e),
        };

        let level = LoggingManager::get_log_level();

        println!("Logger level: {}", level);

        let logger: LoggingManager = Self { writer, level };

        //Leak the box to create a static logger reference...
        let leaked: &'static LoggingManager = Box::leak(Box::new(logger));
        LOGGER
            .set(leaked)
            .unwrap_or_else(|e| panic!("Error setting logger"));

        //give log the logger..
        log::set_logger(LOGGER.get().unwrap())?;
        log::set_max_level(LoggingManager::get_log_filter());

        //Panic hook to flush the logger before crash
        panic::set_hook(Box::new(|panic_info| {
            PanicHandler::handle_panic(panic_info)
        }));

        Ok(())
    }

    fn get_log_filter() -> LevelFilter {
        match LoggingManager::get_log_level() {
            Level::Debug => LevelFilter::Debug,
            Level::Trace => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }
    }

    fn get_log_level() -> Level {
        let current;

        if cfg!(feature = "Trace") {
            current = Level::Trace;
        } else if cfg!(feature = "Debug") {
            current = Level::Debug;
        } else {
            current = Level::Info;
        }

        current
    }

    fn time_stamp() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn static_flush() {
        match LOGGER.get() {
            Some(logger) => {
                println!("Perform static flush");
                (*logger).flush();
            }
            None => panic!("Could not access logger"),
        }
    }

    fn message_log(&self, data: LogMessage) {
        let res = self.writer.write_string(&format!("{data:?}"));
        Self::handle_result(res);

        if data.flush {
            self.flush();
        }
    }

    pub fn handle_result(res: Result<usize, std::io::Error>) {
        match res {
            Ok(count) => {
                if count == 0 {
                    panic!("Could not write to file!");
                }

                //println!("Wrote {} bytes", count);
            }
            Err(err_msg) => {
                panic!("Could not write message to file: {}", err_msg)
            }
        }
    }
}

impl log::Log for LoggingManager {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.level >= metadata.level()
    }

    fn log(&self, record: &Record) {
        let data = LogMessageParser::parse(record);
        self.message_log(data);
    }

    fn flush(&self) {
        println!("Flushing log");
        let _ = &self.writer.flush();
    }
}
