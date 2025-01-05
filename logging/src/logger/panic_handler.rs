use std::{backtrace::{self, Backtrace}, panic::PanicInfo, thread::sleep, time::Duration};
use regex::Regex;

use log::error;

use crate::file_handling::string_builder::StringBuilder;

pub struct PanicHandler;



impl PanicHandler
{

    fn split_backtrace(input: String) -> String
    {
        let re = Regex::new(r"\s+at\s+").unwrap();
        re.replace_all(&input, "\\n").to_string()
    }

    fn format_location(panic_info: &PanicInfo<'_>) -> String
    {
        let (file, location) = match panic_info.location()
        {
            Some(loc) => (loc.file().to_string(), loc.line().to_string()),
            None => ("Unknown".to_owned(), "Unknown".to_owned())
        };

        format!("file: {} \n, location: {}", file, location)
    }

    fn format_panic_info(panic_info: &PanicInfo<'_>) -> String
    {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            format!("panic: {}", s)
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            format!("panic: {}", s)
        }else {
            format!("panic info type id {:?}", panic_info.payload().type_id())
        }
    }

    pub fn handle_panic(panic_info: &PanicInfo<'_>)
    {
        let backtrace: Backtrace = Backtrace::capture();

        let message: String = StringBuilder::new()
        .add_line("panic occured  ")
        .add_line(&Self::format_location(panic_info))
        .add_line(&Self::split_backtrace(backtrace.to_string()))
        .add(&Self::format_panic_info(panic_info)).into();

        error!("{}", message);
        sleep(Duration::from_millis(5));
        println!("{}", message);
    }
}