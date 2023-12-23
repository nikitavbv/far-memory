use {
    std::{ops::Deref, fmt::Debug, marker::PhantomData},
    tracing::{span, Level},
    super::{FarMemoryClient, span::SpanId},
};

pub struct FarMemoryVec<T> {
    client: FarMemoryClient,
    span: SpanId,
    len: usize,

    _phantom: PhantomData<T>,
}

pub struct FarMemoryLocalVec<T> {
    client: FarMemoryClient,
    span: SpanId,
    vec: Vec<T>,
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
        client.decrease_refs_for_span(&span);

        Self {
            client,
            span,
            len: vec.len(),

            _phantom: PhantomData,
        }
    }

    pub fn to_local_vec(&self) -> FarMemoryLocalVec<T> {
        span!(Level::DEBUG, "FarMemoryVec::to_local_vec", span_id=self.span.id()).in_scope(|| {
            let ptr = self.client.span_ptr(&self.span) as *const T;
            if self.len * std::mem::size_of::<T>() != self.client.span_local_memory_usage(&self.span) {
                panic!("memory needed for mem does not match size of memory allocated");
            }

            FarMemoryLocalVec {
                client: self.client.clone(),
                span: self.span.clone(),
                vec: unsafe { Vec::from_raw_parts(ptr as *mut T, self.len, self.len) },
            }
        })
    }
}

impl<T> Deref for FarMemoryLocalVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T: Debug> Debug for FarMemoryLocalVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: std::cmp::PartialEq> PartialEq<Vec<T>> for FarMemoryLocalVec<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.deref() == other
    }
}

impl<T: std::cmp::PartialEq> PartialEq<FarMemoryLocalVec<T>> for Vec<T> {
    fn eq(&self, other: &FarMemoryLocalVec<T>) -> bool {
        self == other.deref()
    }
}

impl<T> Drop for FarMemoryLocalVec<T> {
    fn drop(&mut self) {
        let mut v = Vec::new();
        std::mem::swap(&mut self.vec, &mut v);
        std::mem::forget(v);
        self.client.decrease_refs_for_span(&self.span);
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

    #[test]
    fn to_local_vec_no_double_free() {
        let vec = FarMemoryVec::from_vec(
            FarMemoryClient::new(Box::new(InMemoryBackend::new()), 1000),
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02]
        );

        assert_eq!(
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02],
            vec.to_local_vec()
        );

        assert_eq!(
            vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02],
            vec.to_local_vec()
        );
    }
}
