use std::{collections::BTreeMap, fmt::Debug, str::FromStr};

use log::{
    kv::{Key, Value, VisitSource}, Level, Metadata, Record
};

//not needed to write the messages to the buffer,
//but its here for serialization / deserialization...

#[derive(Debug)]
pub struct LogMessage
{
    pub level: Level,
    pub target: String,
    pub file: String,
    pub line: u32,
    pub module: String,
    pub flush: bool,
    pub message: String,
    pub data: String
}

/// Parse out a LogMessage from the Log record
/// The only key we are collection right now is the data. 
/// But we could add more in the future...
pub struct LogMessageParser<'kvs>(BTreeMap<Key<'kvs>, Value<'kvs>>);

impl<'kvs> VisitSource<'kvs> for LogMessageParser<'kvs> {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), log::kv::Error> {
        self.0.insert(key, value);

        Ok(())
    }
}

impl<'kvs> LogMessageParser<'kvs> {
    pub fn new() -> Self {
        LogMessageParser(BTreeMap::new())
    }

    pub fn parse(record: &Record) -> LogMessage {
        let mut parser: LogMessageParser = LogMessageParser::new();
        match record.key_values().visit(&mut parser)
        {
            Ok(_) => {},
            Err(e) => println!("Failed to parse record: {}", e)
        }

        parser.get_next_packet(record)
    }

    pub fn get_next_packet(&self, record: &Record) -> LogMessage {
        let collected = &self.0;

        let level: Level = record.metadata().level();
        let target: String = record.metadata().target().to_string();
        let file: String = record.file().unwrap_or(&"Unknown").to_string();
        let line: u32 = record.line().unwrap_or(0);
        let module = record.module_path().unwrap_or(&"Unknown").to_string();
        let message: String = record.args().to_string();
        let flush = match collected.get("flush")
        {
            Some(val) => val.to_bool().unwrap_or(false),
            None => false
        };

        let data: String = match collected.get("data")
        {
            Some(val) => val.to_string(),
            None => "".to_string()
        };

        LogMessage {
            level,
            target,
            file,
            line,
            module,
            flush,
            message,
            data
        }
    }
}
