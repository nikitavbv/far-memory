use {
    std::ops::Deref,
    serde::{Serialize, Deserialize},
};

pub struct FarMemory<T> {
    inner: FarMemoryInner<T>,
}

enum FarMemoryInner<T> {
    Local(T),
    Remote,
}

impl<'de, T: Serialize + Deserialize<'de>> FarMemory<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: FarMemoryInner::Local(inner),
        }
    }
}

impl<T> Deref for FarMemory<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.inner {
            FarMemoryInner::Local(t) => t,
            FarMemoryInner::Remote => panic!("oops, it is remote!!!"),
        }
    }
}
