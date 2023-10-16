use crate::client::span::SpanId;

pub mod disk;
pub mod in_memory;
pub mod network_node;

pub trait FarMemoryBackend {
    fn swap_out(&self, id: SpanId, span: &[u8]);
    fn swap_in(&self, id: &SpanId) -> Vec<u8>;
}