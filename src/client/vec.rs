use {
    std::marker::PhantomData,
    super::{
        FarMemoryClient,
        buffer::FarMemoryBuffer,
    },
};

// far memory vec
pub struct FarMemoryVec<T> {
    buffer: FarMemoryBuffer,
    len: usize,

    _phantom: PhantomData<T>,
}

impl<T> FarMemoryVec<T> {
    pub fn new(client: FarMemoryClient) -> Self {
        Self {
            buffer: FarMemoryBuffer::new(client),
            len: 0,

            _phantom: PhantomData,
        }
    }

    pub fn from_vec(client: FarMemoryClient, vec: Vec<T>) -> Self {
        let mut v = Self::new(client);
        v.append(vec);
        v
    }

    pub fn swap_out(&self) {
        self.buffer.swap_out();
    }

    pub fn to_local_vec(&self) -> Vec<T> {
        let size = std::mem::size_of::<T>();

        let data = self.buffer.slice(0..(self.len() * size));

        unsafe {
            let res = Vec::from_raw_parts(data.as_ptr() as *mut T, self.len(), self.len());
            std::mem::forget(data); // to prevent double free
            res
        }
    }

    pub fn append(&mut self, vec: Vec<T>) {
        for item in vec {
            self.push(item);
        }
    }

    pub fn push(&mut self, item: T) {
        let ptr = &item as *const _ as *const u8;
        let data = unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of_val(&item)) }.to_vec();
        self.buffer.append(data);
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> T {
        let size = std::mem::size_of::<T>();

        let entry = self.buffer.slice((index * size)..((index + 1) * size));
        unsafe {
            std::ptr::read(entry.as_ptr() as *const _)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::client::InMemoryBackend,
        super::*,
    };

    #[test]
    fn get() {
        let vec = FarMemoryVec::from_vec(
            FarMemoryClient::new(Box::new(InMemoryBackend::new())), 
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02]
        );
        
        assert_eq!(10, vec.len());
        assert_eq!(10.02, vec.get(0));
        assert_eq!(9.02, vec.get(1));
        assert_eq!(8.02, vec.get(2));
        assert_eq!(7.02, vec.get(3));
        assert_eq!(6.02, vec.get(4));
        assert_eq!(5.02, vec.get(5));
        assert_eq!(4.02, vec.get(6));
        assert_eq!(3.02, vec.get(7));
        assert_eq!(2.02, vec.get(8));
        assert_eq!(1.02, vec.get(9));
    }

    #[test]
    fn to_local_vec() {
        let vec = FarMemoryVec::from_vec(
            FarMemoryClient::new(Box::new(InMemoryBackend::new())), 
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02]
        );
        
        assert_eq!(
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02],
            vec.to_local_vec()
        );
    }
}