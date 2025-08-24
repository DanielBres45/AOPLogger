use std::time::Instant;

use log::Level;
use log::{log, trace};

pub struct MethodTracer {
    file_name: String,
    line_number: u32,
    start: Instant,
}

impl MethodTracer {
    pub fn new(file_name: String, line_number: u32) -> Self {
        MethodTracer {
            file_name,
            line_number,
            start: Instant::now(),
        }
    }

    pub fn dispose(&self) {

        log!(
            target: "MethodTracer",
            Level::Trace,
            "{:?}", self.start.elapsed()
        );
    }
}
