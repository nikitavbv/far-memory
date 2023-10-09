use {
    std::{sync::{Arc, atomic::{AtomicU64, Ordering}, RwLock}, collections::HashMap, alloc::{GlobalAlloc, Layout}},
    crate::utils::allocator::GLOBAL,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SpanId(u64);

#[derive(Clone)]
pub struct FarMemoryClient {
    span_id_counter: Arc<AtomicU64>,
    spans: Arc<RwLock<HashMap<SpanId, FarMemorySpan>>>,
}

enum FarMemorySpan {
    Local(*mut u8),
    Remote,
}

impl FarMemoryClient {
    pub fn new() -> Self {
        Self {
            span_id_counter: Arc::new(AtomicU64::new(0)),
            spans: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn allocate_span(&self) -> SpanId {
        let id = SpanId(self.span_id_counter.fetch_add(1, Ordering::Relaxed));
        self.spans.write().unwrap().insert(id.clone(), self.new_span());
        id
    }

    fn new_span(&self) -> FarMemorySpan {
        let layout = Layout::array::<u8>(self.span_size()).unwrap();
        FarMemorySpan::Local(unsafe { GLOBAL.alloc(layout) })
    }

    pub fn span_ptr(&self, id: &SpanId) -> *mut u8 {
        // move it to local memory if needed
        unimplemented!()
    }

    pub fn span_size(&self) -> usize {
        2 * 1024 * 1024 // 2MB
    }
}