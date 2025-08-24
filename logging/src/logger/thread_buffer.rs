use std::{
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write},
    sync::{Arc, Mutex},
};

use serde::Serialize;

pub struct ThreadSafeBufferedWriter<T: Sized + Write> {
    writer: Arc<Mutex<BufWriter<T>>>,
}

impl<T: Sized + Write> Drop for ThreadSafeBufferedWriter<T> {
    fn drop(&mut self) {
        drop(self.writer.lock().unwrap())
    }
}

impl ThreadSafeBufferedWriter<File> {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;
        let writer = BufWriter::new(file);
        Ok(ThreadSafeBufferedWriter {
            writer: Arc::new(Mutex::new(writer)),
        })
    }
}

impl<T: Sized + Write> ThreadSafeBufferedWriter<T> {
    pub fn write_string(&self, data: &str) -> io::Result<usize> {
        let mut writer = self.writer.lock().unwrap();

        let mut tot: usize = 0;
        for line in data.split('\n') {
            match writer.write(format!("{}\n", line).as_bytes()) {
                Ok(v) => tot += v,
                Err(e) => return Err(e),
            }
        }

        return Ok(tot);
    }

    pub fn write_json<I: Sized + Serialize>(&self, data: I) -> Result<(), serde_json::Error> {
        serde_json::to_writer(&mut *self.writer.lock().unwrap(), &data)
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
