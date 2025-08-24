use regex::Regex;
use std::{backtrace::Backtrace, panic::PanicHookInfo, thread::sleep, time::Duration};

use log::error;

pub struct PanicHandler;

impl PanicHandler {
    fn split_backtrace(input: String) -> String {
        let re = Regex::new(r"\s+at\s+").unwrap();
        re.replace_all(&input, "\\n").to_string()
    }

    fn format_location(panic_info: &PanicHookInfo<'_>) -> String {
        let (file, location) = match panic_info.location() {
            Some(loc) => (loc.file().to_string(), loc.line().to_string()),
            None => ("Unknown".to_owned(), "Unknown".to_owned()),
        };

        format!("file: {} \n, location: {}", file, location)
    }

    fn format_panic_info(panic_info: &PanicHookInfo<'_>) -> String {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            format!("panic: {}", s)
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            format!("panic: {}", s)
        } else {
            format!("panic info type id {:?}", panic_info.payload().type_id())
        }
    }

    pub fn handle_panic(panic_info: &PanicHookInfo<'_>) {
        let backtrace: Backtrace = Backtrace::capture();

        let mut message: String = "panic occured  \n".to_string();
        message.push_str(&Self::format_location(panic_info));
        message.push_str(&Self::split_backtrace(backtrace.to_string()));
        message.push_str(&Self::format_panic_info(panic_info));

        error!("{}", message);
        sleep(Duration::from_millis(5));
        println!("{}", message);
    }
}
