use {
    std::marker::PhantomData,
    super::{FarMemoryClient, client::SpanId},
};

pub struct FarMemoryVec<T> {
    client: FarMemoryClient,
    span: SpanId,
    len: usize,

    _phantom: PhantomData<T>,
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

            _phantom: PhantomData,
        }
    }

    pub fn to_local_vec(&self) -> Vec<T> {
        let ptr = self.client.span_ptr(&self.span);

        unsafe {
            Vec::from_raw_parts(ptr as *mut T, self.len, self.len)
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
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02],
            vec.to_local_vec()
        );
    }
}