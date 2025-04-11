use std::{
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write},
    sync::{Arc, Mutex},
};

pub struct ThreadSafeBufferedLogger {
    writer: Arc<Mutex<BufWriter<File>>>,
}

impl Drop for ThreadSafeBufferedLogger {
    fn drop(&mut self) {
        drop(self.writer.lock().unwrap())
    }
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

    pub fn write_string(&self, data: &str) -> io::Result<usize> {
        let mut writer = self.writer.lock().unwrap();

        let mut tot: usize = 0;
        for line in data.split('\n') {
            match writer.write(format!("{}", line).as_bytes()) {
                Ok(v) => tot += v,
                Err(e) => return Err(e),
            }
        }

        return Ok(tot);
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
