use std::time::Instant;

use log::trace;

use super::method_signature::LogHeader;

pub struct MethodTracer{
    signature: LogHeader,
    start: Instant
}

impl MethodTracer{
    pub fn new(name: &str, line: u32) -> Self{
        let signature = LogHeader::build(name, line);

        MethodTracer{
            signature,
            start: Instant::now()
        }
    }

    pub fn dispose(&self) {
        let duration = &self.start.elapsed(); 
        trace!(target = "MethodTracer", file_name = &self.signature.file_name.as_str(), line_number = &self.signature.line_number; "{:?}", duration); 
    }
}

