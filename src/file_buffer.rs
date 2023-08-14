use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::prelude::FileExt;
use std::path::Path;

pub struct FileBuffer {
    file: File,
    buffer: Vec<u8>,
    pos: u64,
    read: u64,
    chunk_size: usize
}

impl FileBuffer {

    pub fn new(path: &str, chunk_size: usize) -> Result<Self, std::io::Error> {
        let file = File::open(Path::new(path))?;
        let buffer = vec![0u8; chunk_size];
        let pos = 0;
        let read: u64 = 0;
        Ok(Self {
            file,
            buffer,
            pos,
            read,
            chunk_size
        })
    }

    // Read bytes in a rush till buffer is full!
    #[allow(dead_code)]
    pub fn read_bytes(&mut self, buf: &mut [u8], start: u64) {
        self.file.read_exact_at(buf, start).expect("Error reading bytes!");
    }

    fn align(&mut self, i: u64) -> u64 {
        i - (i % self.chunk_size as u64)
    }

    pub fn get(&mut self, i: u64) -> Option<u8> {
        if i < self.pos || i >= (self.pos + self.read) {
             self.pos = self.align(i);
             self.file.seek(SeekFrom::Start(self.pos as u64)).expect("Error seeking in File!");
            self.read = self.file.read(&mut self.buffer).ok().expect("Error") as u64;
        }
        Some(self.buffer[(i - self.pos) as usize])
    }

    #[allow(dead_code)]
    pub fn print_buffer_content(&mut self) {
        let cont = String::from_utf8_lossy(&self.buffer);
        println!("{}", cont);
    }

    #[allow(dead_code)]
    pub fn get_size(&self) -> u64{
        self.file.metadata().unwrap().len()
    }


}