use std::{fs::OpenOptions, io::{self, BufWriter, Write}, sync::{Arc, Mutex}};




pub struct ThreadSafeBufferedLogger {
    writer: Arc<Mutex<BufWriter<std::fs::File>>>,
}

impl ThreadSafeBufferedLogger {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        let writer = BufWriter::new(file);
        Ok(ThreadSafeBufferedLogger {
            writer: Arc::new(Mutex::new(writer)),
        })
    }

    pub fn write_string(&self, data: &str){ 
        let mut writer = self.writer.lock().unwrap();
        for line in data.split('\n')
        {
            _ = writeln!(writer, "{}", line)
        }
    }

    pub fn write(&self, data: &[u8]) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap();
        writer.write_all(data)
        
    }

    pub fn flush(&self) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap();
        writer.flush()
    }
}