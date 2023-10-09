use {
    std::{ops::Deref, collections::HashMap, marker::PhantomData, sync::Arc},
    serde::{Serialize, Deserialize},
};

pub use self::{
    buffer::FarMemoryBuffer,
    client::FarMemoryClient,
};

pub mod backend;

mod buffer;
mod client;

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

// far memory vec
pub struct FarMemoryVec<T> {
    _phantom: PhantomData<T>,
}

impl<T> FarMemoryVec<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}