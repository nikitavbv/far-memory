use crate::client::span::SpanId;

pub mod compression;
pub mod disk;
pub mod encryption;
pub mod erasure_coding;
pub mod in_memory;
pub mod metrics;
pub mod network_node;
pub mod network_sharding;
pub mod replication;

pub trait FarMemoryBackend: Send + Sync {
    // far memory backend performs prepend, so when swapping in we can append to existing memory, which is
    //  faster.
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool);
    fn swap_in(&self, id: &SpanId) -> Vec<u8>;

    fn batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) {
        self.batch(swap_out_operations, None);
    }

    fn batch(&self, swap_out_operations: Vec<SwapOutOperation>, swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        swap_out_operations.iter().for_each(|op| self.swap_out(op.id.clone(), op.data.as_slice(), op.prepend));
        swap_in.map(|v| self.swap_in(&v))
    }

    fn on_stop(&self) {}
}

pub struct SwapOutOperation {
    id: SpanId,
    data: SwapOutOperationData,
    prepend: bool,
}

impl SwapOutOperation {
    pub fn new(id: SpanId, data: SwapOutOperationData, prepend: bool) -> Self {
        Self {
            id,
            data,
            prepend,
        }
    }
}

pub enum SwapOutOperationData {
    Owned(Vec<u8>),
    ReadFrom {
        ptr: *mut u8,
        size: usize,
    }
}

impl SwapOutOperationData {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Owned(data) => &data,
            Self::ReadFrom { ptr, size } => unsafe {
                std::slice::from_raw_parts(*ptr, *size)
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Owned(data) => data.len(),
            Self::ReadFrom { ptr: _, size } => *size,
        }
    }
}
