use std::collections::BTreeMap;

use log::kv::{Key, Value, VisitSource};

use super::{message_data::MessageHeader, method_signature::LogHeader};

///Parses the log message header, contains the formatted keys like
/// MethodSignature, Target, level
pub struct KeyCollector<'kvs>(BTreeMap<Key<'kvs>, Value<'kvs>>);

impl<'kvs> VisitSource<'kvs> for KeyCollector<'kvs> {
    fn visit_pair(&mut self, key: Key<'kvs>, value: Value<'kvs>) -> Result<(), log::kv::Error> {
        self.0.insert(key, value);

        Ok(())
    }
}

impl<'kvs> KeyCollector<'kvs>{

    pub fn new() -> Self{
        KeyCollector(BTreeMap::new())
    }

    pub fn get_next_header(&self) -> MessageHeader{
        let collected = &self.0;

        let mut name: Option<String> = None;
        let mut line: u32 = 0;
        for (key, value) in collected.into_iter(){
            if key.as_str() == "file_name"{
                name = Some(value.to_string());
            }

            if key.as_str() == "line_number"{
                line = u32::try_from(value.to_u64().unwrap()).ok().unwrap_or(0);
            }
        }

        let method_signature = match !name.is_none() && line != 0{
            true => LogHeader::build(&name.unwrap(), line),
            false => LogHeader::default()
        };

        MessageHeader{
            method_signature
        }
    }
}
