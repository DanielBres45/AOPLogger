use log::Record;

use super::{key_collector::KeyCollector, method_signature::LogHeader};


pub struct MessageHeader{
    pub method_signature: LogHeader
}

impl Default for MessageHeader{
    fn default() -> Self{
        MessageHeader{
            method_signature: LogHeader::default()
        }
    }
}

pub struct MessageData<'a>{
    pub header: MessageHeader,
    pub target: &'a str,
    pub message: String
}

impl<'a> MessageData<'a>{
    pub fn parse(record: &'a Record) -> Self{

        let mut visitor = KeyCollector::new();

        let _ = record.key_values().visit(&mut visitor);

        let header = visitor.get_next_header();
        let message = record.args().to_string();

        let target = record.target();

        MessageData{
            header,
            target,
            message
        }
    }
}