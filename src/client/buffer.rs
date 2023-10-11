use {
    std::ops::{Index, Range},
    super::client::{FarMemoryClient, SpanId},
};

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

    pub fn swap_out(&self) {
        self.client.swap_out_spans(&self.spans);
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

        let ptr = self.client.span_ptr(&self.spans[self.spans.len() - 1]);
        let offset = self.len % self.client.span_size();

        unsafe {
            let src = bytes as *const _ as *const u8;
            let dst = ptr.offset(offset as isize);
            std::ptr::copy(src, dst, bytes.len());
        }

        self.len += bytes.len();
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn total_capacity(&self) -> usize {
        self.spans.len() * self.client.span_size()
    }

    pub fn slice(&self, range: Range<usize>) -> Vec<u8> {
        let mut i = range.start;
        let mut result = vec![0; range.len()];

        let span_size = self.client.span_size();

        while i < range.end {
            let span_index = i / span_size;
            let span_offset = i % span_size;

            let ptr = self.client.span_ptr(&self.spans[span_index]);
            let bytes_to_read = (span_size - span_offset).min(range.end - i);

            unsafe {
                std::ptr::copy(ptr.offset(span_offset as isize), result.as_mut_ptr().offset((i - range.start) as isize), bytes_to_read);
            }
            i += bytes_to_read;            
        }

        result
    }

    pub fn ensure_local_memory_under_limit(&self) {
        // TODO: remove this. Memory limit should be enforced on swap in.
        self.client.ensure_local_memory_under_limit();
    }
}

impl Index<usize> for FarMemoryBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        let span_size = self.client.span_size();
        let span_index = index / span_size;
        let span_offset = index % span_size;

        let ptr = self.client.span_ptr(&self.spans[span_index]);
        
        // todo: lock it somehow to prevent swap out? Probably can lock using smart pointers.
        unsafe {
            let ptr = ptr.offset(span_offset as isize);
            &*ptr
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::client::InMemoryBackend,
        super::*,
    };

    #[test]
    fn index() {
        let client = FarMemoryClient::new(Box::new(InMemoryBackend::new()), 1000);
        let buffer = FarMemoryBuffer::from_bytes(client, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);

        assert_eq!(10, buffer.len());

        assert_eq!(10, buffer[0]);
        assert_eq!(9, buffer[1]);
        assert_eq!(8, buffer[2]);
        assert_eq!(7, buffer[3]);
        assert_eq!(6, buffer[4]);
        assert_eq!(5, buffer[5]);
        assert_eq!(4, buffer[6]);
        assert_eq!(3, buffer[7]);
        assert_eq!(2, buffer[8]);
        assert_eq!(1, buffer[9]);
    }

    #[test]
    fn slice() {
        let client = FarMemoryClient::new(Box::new(InMemoryBackend::new()), 1000);
        let buffer = FarMemoryBuffer::from_bytes(client, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);

        assert_eq!(vec![7, 6, 5], buffer.slice(3..6));
    }

    #[test]
    fn append_twice() {
        let mut buffer = FarMemoryBuffer::new(FarMemoryClient::new(Box::new(InMemoryBackend::new()), 1000));
        buffer.append(vec![1, 2, 3]);
        buffer.append(vec![4, 5, 6]);

        assert_eq!(vec![1, 2, 3, 4, 5, 6], buffer.slice(0..buffer.len()));
    }
}