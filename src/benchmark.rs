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