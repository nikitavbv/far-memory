use {
    std::{sync::{Arc, atomic::{AtomicU64, Ordering}, RwLock}, collections::HashMap, alloc::{GlobalAlloc, Layout}},
    crate::utils::allocator::GLOBAL,
    super::backend::in_memory::InMemoryBackend,
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SpanId(u64);

#[derive(Clone)]
pub struct FarMemoryClient {
    span_id_counter: Arc<AtomicU64>,
    spans: Arc<RwLock<HashMap<SpanId, FarMemorySpan>>>,

    backend: Arc<InMemoryBackend>,
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

            backend: Arc::new(InMemoryBackend::new()),
        }
    }

    pub fn allocate_span(&self) -> SpanId {
        let id = SpanId(self.span_id_counter.fetch_add(1, Ordering::Relaxed));
        self.spans.write().unwrap().insert(id.clone(), self.new_span());
        id
    }

    fn new_span(&self) -> FarMemorySpan {
        FarMemorySpan::Local(unsafe { GLOBAL.alloc(self.span_layout()) })
    }

    pub fn span_ptr(&self, id: &SpanId) -> *mut u8 {
        let span = &self.spans.read().unwrap()[id];
        match span {
            FarMemorySpan::Local(addr) => addr.clone(),
            FarMemorySpan::Remote => unimplemented!(),
        }
    }

    fn read_span_ptr_to_slice(&self, span_ptr: *mut u8) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(span_ptr, self.span_size())
        }
    }

    pub fn span_size(&self) -> usize {
        2 * 1024 * 1024 // 2MB
    }

    fn span_layout(&self) -> Layout {
        Layout::array::<u8>(self.span_size()).unwrap()
    }

    pub fn swap_out_spans(&self, spans: &[SpanId]) {
        for span in spans {
            self.swap_out_span(span)
        }
    }

    fn swap_out_span(&self, span_id: &SpanId) {
        let span = self.spans.write().unwrap().insert(span_id.clone(), FarMemorySpan::Remote).unwrap();

        let ptr = match span {
            FarMemorySpan::Local(addr) => addr.clone(),
            FarMemorySpan::Remote => return,
        };

        let data = self.read_span_ptr_to_slice(ptr);

        self.backend.swap_out(span_id.clone(), data);
        
        self.spans.write().unwrap().insert(span_id.clone(), FarMemorySpan::Remote);

        unsafe {
            GLOBAL.dealloc(ptr, self.span_layout());
        }
    }
}
