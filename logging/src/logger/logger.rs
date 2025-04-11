use std::panic;

use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

use crate::file_handling::thread_buffer::ThreadSafeBufferedLogger;

use super::message_data::MessageData;
use super::panic_handler::PanicHandler;

pub struct LoggingManager {
    writer: ThreadSafeBufferedLogger,
    level: Level,
}

impl Drop for LoggingManager {
    fn drop(&mut self) {
        self.flush();
    }
}

impl LoggingManager {
    pub fn init(file_path: &str) -> Result<(), SetLoggerError> {
        let writer = match ThreadSafeBufferedLogger::new(file_path) {
            Ok(buf) => buf,
            Err(e) => panic!("Failed to create new buffered logger {}", e),
        };

        let level = LoggingManager::get_log_level();

        println!("Logger level: {}", level);

        let logger: LoggingManager = Self { writer, level };

        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(LoggingManager::get_log_filter());

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

    fn panic_log(&self, data: MessageData) {
        let _ = &self.writer.write_string(&data.message);

        let _ = &self.writer.flush();
    }

    pub fn flush(&self) {
        let _ = &self.writer.flush();
    }

    fn message_log(&self, data: MessageData) {
        let message = format!(
            "[{:?}] Target: {} - {}, Message: {:?}\n",
            LoggingManager::time_stamp(),
            data.target,
            data.header.method_signature,
            data.message
        );

        let res = self.writer.write_string(&message);
        Self::handle_result(res);

        if data.header.flush {
            self.flush();
        }
    }

    pub fn handle_result(res: Result<usize, std::io::Error>) {
        match res {
            Ok(count) => {
                if count == 0 {
                    panic!("Could not write to file!");
                }

                println!("Wrote {} bytes", count);
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
        if self.enabled(record.metadata()) {
            let data = MessageData::parse(record);

            match record.metadata().level() {
                Level::Error => self.panic_log(data),
                Level::Debug | Level::Info | Level::Trace | Level::Warn => self.message_log(data),
            }
        }
    }

    fn flush(&self) {}
}
