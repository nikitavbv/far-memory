use {
    std::cell::UnsafeCell,
    tracing::{span, Level},
    super::{FarMemoryClient, client::SpanId},
};

pub struct FarMemoryVec<T> {
    client: FarMemoryClient,
    span: SpanId,
    len: usize,
    vec: UnsafeCell<Vec<T>>,
}

impl <T> FarMemoryVec<T> {
    pub fn from_vec(client: FarMemoryClient, vec: Vec<T>) -> Self {
        let size = std::mem::size_of::<T>() * vec.len();
        let span = client.allocate_span(size);
        let ptr = client.span_ptr(&span);
        // this can probably be optimized by taking ptr and giving it to client, instead of
        // allocation and copy
        unsafe {
            std::ptr::copy_nonoverlapping(vec.as_ptr() as *const _, ptr, size);
        }

        Self {
            client,
            span,
            len: vec.len(),
            vec: UnsafeCell::new(unsafe {
                Vec::from_raw_parts(ptr as *mut T, vec.len(), vec.len())
            }),
        }
    }

    pub fn to_local_vec(&self) -> &Vec<T> {
        span!(Level::DEBUG, "FarMemoryVec::to_local_vec").in_scope(|| {
            let ptr = self.client.span_ptr(&self.span) as *const T;
            unsafe {
                if ptr != (*self.vec.get()).as_ptr() {
                    let mut t = Vec::from_raw_parts(ptr as *mut T, self.len, self.len);
                    std::mem::swap(&mut *self.vec.get(), &mut t);
                    std::mem::forget(t);
                }
    
                & *self.vec.get()
            }
        })        
    }

    pub fn ensure_local_memory_under_limit(&self) {
        span!(Level::DEBUG, "FarMemoryVec::ensure_local_memory_under_limit").in_scope(|| {
            // TODO: remove this. Memory limit should be enforced on swap in.
            self.client.ensure_local_memory_under_limit();
        });
    }
}

impl<T> Drop for FarMemoryVec<T> {
    fn drop(&mut self) {
        unsafe {
            let mut t = Vec::new();
            std::mem::swap(&mut *self.vec.get(), &mut t);
            std::mem::forget(t);
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
    fn to_local_vec() {
        let vec = FarMemoryVec::from_vec(
            FarMemoryClient::new(Box::new(InMemoryBackend::new()), 1000), 
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02]
        );
        
        assert_eq!(
            &vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02],
            vec.to_local_vec()
        );
    }

    #[test]
    fn to_local_vec_no_double_free() {
        let vec = FarMemoryVec::from_vec(
            FarMemoryClient::new(Box::new(InMemoryBackend::new()), 1000), 
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02]
        );
        
        assert_eq!(
            &vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02],
            vec.to_local_vec()
        );

        assert_eq!(
            &vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02],
            vec.to_local_vec()
        );
    }
}