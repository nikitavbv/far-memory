use {
    std::{sync::{Arc, atomic::{AtomicU64, Ordering}, RwLock}, collections::HashMap, alloc::{GlobalAlloc, Layout}},
    crate::utils::allocator::GLOBAL,
    super::backend::{
        FarMemoryBackend,
        in_memory::InMemoryBackend,
    },
};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SpanId(u64);

#[derive(Clone)]
pub struct FarMemoryClient {
    span_id_counter: Arc<AtomicU64>,
    spans: Arc<RwLock<HashMap<SpanId, FarMemorySpan>>>,

    backend: Arc<Box<dyn FarMemoryBackend>>,
}

enum FarMemorySpan {
    Local(*mut u8),
    Remote,
}

impl SpanId {
    pub fn id(&self) -> u64 {
        self.0
    }
}

impl FarMemoryClient {
    pub fn new(backend: Box<dyn FarMemoryBackend>) -> Self {
        Self {
            span_id_counter: Arc::new(AtomicU64::new(0)),
            spans: Arc::new(RwLock::new(HashMap::new())),

            backend: Arc::new(backend),
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
        {
            let span = &self.spans.read().unwrap()[id];
            match span {
                FarMemorySpan::Local(addr) => return addr.clone(),
                FarMemorySpan::Remote => {
                    // will need to swap in
                },
            };
        }

        // swap in
        let data = self.backend.swap_in(id);

        let ptr = unsafe {
            GLOBAL.alloc(self.span_layout())
        };
        unsafe {
            std::ptr::copy(data.as_slice() as *const _ as *const u8, ptr, data.len());
        }

        self.spans.write().unwrap().insert(id.clone(), FarMemorySpan::Local(ptr.clone()));

        ptr
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

    pub fn total_local_spans(&self) -> usize {
        self.spans.read().unwrap().iter().filter(|v| v.1.is_local()).count()
    }

    pub fn total_remote_spans(&self) -> usize {
        self.spans.read().unwrap().len() - self.total_local_spans()
    }
}

impl FarMemorySpan {
    pub fn is_local(&self) -> bool {
        match self {
            FarMemorySpan::Local(_) => true,
            FarMemorySpan::Remote => false,
        }
    }
}