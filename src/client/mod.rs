use {
    std::ops::Deref,
    serde::{Serialize, Deserialize},
};

pub use self::{
    buffer::FarMemoryBuffer,
    buffered_vec::FarMemoryBufferedVec,
    vec::FarMemoryVec,
    client::FarMemoryClient,
    span::SpanId,
    backend::{
        FarMemoryBackend,
        in_memory::InMemoryBackend,
        disk::LocalDiskBackend,
        network_node::NetworkNodeBackend,
    },
};

pub mod backend;

mod buffer;
mod buffered_vec;
mod client;
mod span;
mod vec;

/**
 * - far memory object needs to have reference to client to read and write using it.
 * - actual data should be stored in client, so it can be managed there.
 */

// far memory object
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
