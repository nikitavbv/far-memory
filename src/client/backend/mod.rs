use crate::client::client::SpanId;

pub mod in_memory;

pub trait FarMemoryBackend {
    fn swap_out(&self, id: SpanId, span: &[u8]);
    fn swap_in(&self, id: &SpanId) -> Vec<u8>;
}