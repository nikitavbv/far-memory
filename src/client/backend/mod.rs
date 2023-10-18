use crate::client::span::SpanId;

pub mod disk;
pub mod in_memory;
pub mod network_node;

pub trait FarMemoryBackend {
    // far memory backend performs prepend, so when swapping in we can append to existing memory, which is 
    // faster.
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool);
    fn swap_in(&self, id: &SpanId) -> Vec<u8>;
}