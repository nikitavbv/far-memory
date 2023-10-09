use super::client::{FarMemoryClient, SpanId};

pub struct FarMemoryBuffer {
    client: FarMemoryClient,
    spans: Vec<SpanId>,
    len: usize,
}

impl FarMemoryBuffer {
    pub fn new(client: FarMemoryClient) -> Self {
        Self {
            client,
            spans: Vec::new(),
            len: 0,
        }
    }

    pub fn from_bytes(client: FarMemoryClient, bytes: Vec<u8>) -> Self {
        let mut buffer = Self::new(client);
        buffer.append(bytes);
        buffer
    }

    pub fn append(&mut self, bytes: Vec<u8>) {
        let mut i = 0;
    
        while i < bytes.len() {
            let free_space = self.total_capacity() - self.len;
            let len_to_add = free_space.min(bytes.len());
            self.append_to_last_span(&bytes[i..(i + len_to_add)]);
            i += len_to_add;

            if i < bytes.len() {
                self.grow();
            }
        }
    }

    fn grow(&mut self) {
        self.spans.push(self.client.allocate_span())
    }

    fn append_to_last_span(&mut self, bytes: &[u8]) {
        if bytes.len() == 0 {
            return;
        }

        unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn total_capacity(&self) -> usize {
        self.spans.len() * self.client.span_size()
    }
}