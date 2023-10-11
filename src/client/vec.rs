use {
    std::{marker::PhantomData, time::Instant, sync::atomic::{AtomicU64, Ordering}},
    super::{
        FarMemoryClient,
        buffer::FarMemoryBuffer,
    },
};

static TIMER_TO_LOCAL_VEC: AtomicU64 = AtomicU64::new(0);
static TIMER_ENSURE_MEMORY_LIMIT: AtomicU64 = AtomicU64::new(0);

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
        let started_at = Instant::now();

        let size = std::mem::size_of::<T>();

        let data = self.buffer.slice(0..(self.len() * size));

        unsafe {
            let res = Vec::from_raw_parts(data.as_ptr() as *mut T, self.len(), self.len());
            std::mem::forget(data); // to prevent double free
            TIMER_TO_LOCAL_VEC.fetch_add((Instant::now() - started_at).as_millis() as u64, Ordering::Relaxed);
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

    pub fn ensure_local_memory_under_limit(&self) {
        let started_at = Instant::now();
        // TODO: remove this. Memory limit should be enforced on swap in.
        self.buffer.ensure_local_memory_under_limit();
        TIMER_ENSURE_MEMORY_LIMIT.fetch_add((Instant::now() - started_at).as_millis() as u64, Ordering::Relaxed);
    }
}

pub fn print_far_vec_performance_report() {
    println!("far vec performance: to_local_vec {}, ensure_memory_limit {}", TIMER_TO_LOCAL_VEC.load(Ordering::Relaxed), TIMER_ENSURE_MEMORY_LIMIT.load(Ordering::Relaxed));
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
            FarMemoryClient::new(Box::new(InMemoryBackend::new()), 1000), 
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
            FarMemoryClient::new(Box::new(InMemoryBackend::new()), 1000), 
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02]
        );
        
        assert_eq!(
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02],
            vec.to_local_vec()
        );
    }
}