use std::{fs::File, io::{Write, prelude::*}};

use redis::Commands;

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

pub struct RemoteBackend {
    client: redis::Client,
    connection: redis::Connection,
    buffer: Vec<u8>,
}

impl RemoteBackend {
    pub fn new(addr: &str) -> Self {
        let client = redis::Client::open(addr).unwrap();
        let connection = client.get_connection().unwrap();
        Self {
            client,
            connection,
            buffer: Vec::new(),
        }
    }
}

impl IOBackend for RemoteBackend {
    fn write(&mut self, slot: usize, data: Vec<u8>) {
        self.connection.set::<String, Vec<u8>, ()>(format!("far_memory::{}", slot), data).unwrap();
    }

    fn read(&mut self, slot: usize) -> &[u8] {
        self.buffer = self.connection.get::<String, Vec<u8>>(format!("far_memory::{}", slot)).unwrap();
        &self.buffer
    }
}