use std::{fs::File, io::{Write, prelude::*}};

pub trait IOBackend {
    fn write(&mut self, slot: usize, data: Vec<u8>);
    fn read(&mut self, slot: usize) -> &[u8];
}

pub struct InMemoryBackend {
    data: Vec<Box<Vec<u8>>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
}

impl IOBackend for InMemoryBackend {
    fn write(&mut self, slot: usize, data: Vec<u8>) {
        while slot >= self.data.len() {
            self.data.push(Box::new(Vec::new()));
        }
        self.data.insert(slot, Box::new(data));
    }

    fn read(&mut self, slot: usize) -> &[u8] {
        &*self.data[slot]
    }
}

pub struct OnDiskBackend { 
    path_prefix: String,
    buffer: Vec<u8>,
}

impl OnDiskBackend {
    pub fn new(path_prefix: String) -> Self {
        Self {
            path_prefix,
            buffer: Vec::new(),
        }
    }
}

impl IOBackend for OnDiskBackend {
    fn write(&mut self, slot: usize, data: Vec<u8>) {
        let mut file = File::create(format!("{}/{}", self.path_prefix, slot)).unwrap();
        file.write_all(&data).unwrap();
    }

    fn read(&mut self, slot: usize) -> &[u8] {
        let mut file = File::open(format!("{}/{}", self.path_prefix, slot)).unwrap();
        self.buffer.clear();
        file.read_to_end(&mut self.buffer).unwrap();
        &self.buffer
    }
}