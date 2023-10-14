use {
    std::{sync::{Arc, atomic::{AtomicU64, Ordering}, RwLock}, collections::HashMap, alloc::{GlobalAlloc, Layout}},
    crate::utils::allocator::GLOBAL,
    super::backend::{
        FarMemoryBackend,
        in_memory::InMemoryBackend,
    },
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SpanId(u64);

#[derive(Clone)]
pub struct FarMemoryClient {
    span_id_counter: Arc<AtomicU64>,
    spans: Arc<RwLock<HashMap<SpanId, FarMemorySpan>>>,

    backend: Arc<Box<dyn FarMemoryBackend>>,

    local_memory_max_threshold: u64,
}

enum FarMemorySpan {
    Local {
        ptr: *mut u8,
        size: usize,   
    },
    Remote {
        size: usize,
    },
}

impl SpanId {
    pub fn from_id(id: u64) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u64 {
        self.0
    }
}

impl FarMemoryClient {
    pub fn new(backend: Box<dyn FarMemoryBackend>, local_memory_max_threshold: u64) -> Self {
        Self {
            span_id_counter: Arc::new(AtomicU64::new(0)),
            spans: Arc::new(RwLock::new(HashMap::new())),

            backend: Arc::new(backend),
            local_memory_max_threshold,
        }
    }

    pub fn allocate_span(&self, span_size: usize) -> SpanId {
        let id = SpanId(self.span_id_counter.fetch_add(1, Ordering::Relaxed));
        self.spans.write().unwrap().insert(id.clone(), self.new_span(span_size));
        id
    }

    fn new_span(&self, span_size: usize) -> FarMemorySpan {
        FarMemorySpan::Local {
            ptr: unsafe { GLOBAL.alloc(self.span_layout(span_size)) },
            size: span_size,
        }
    }

    pub fn span_ptr(&self, id: &SpanId) -> *mut u8 {
        {
            let span = &self.spans.read().unwrap()[id];
            match span {
                FarMemorySpan::Local { ptr, size: _ } => return ptr.clone(),
                FarMemorySpan::Remote { .. } => {
                    // will need to swap in
                },
            };
        }

        // swap in
        let data = self.backend.swap_in(id);
        let size = data.len();

        let ptr = unsafe {
            GLOBAL.alloc(self.span_layout(size))
        };
        unsafe {
            std::ptr::copy(data.as_slice() as *const _ as *const u8, ptr, size);
        }

        self.spans.write().unwrap().insert(id.clone(), FarMemorySpan::Local {
            ptr: ptr.clone(),
            size,
        });

        ptr
    }

    fn read_span_ptr_to_slice(&self, span_ptr: *mut u8, span_size: usize) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(span_ptr, span_size)
        }
    }

    fn span_layout(&self, span_size: usize) -> Layout {
        Layout::array::<u8>(span_size).unwrap()
    }

    pub fn swap_out_spans(&self, spans: &[SpanId]) {
        for span in spans {
            self.swap_out_span(span)
        }
    }

    fn swap_out_span(&self, span_id: &SpanId) {
        let span = self.spans.write().unwrap().remove(&span_id.clone()).unwrap();

        let (ptr, size) = match span {
            FarMemorySpan::Local { ptr, size } => (ptr.clone(), size),
            FarMemorySpan::Remote { .. } => return,
        };

        let data = self.read_span_ptr_to_slice(ptr, size);

        self.backend.swap_out(span_id.clone(), data);
        
        self.spans.write().unwrap().insert(span_id.clone(), FarMemorySpan::Remote { size });

        unsafe {
            GLOBAL.dealloc(ptr, self.span_layout(size));
        }
    }

    pub fn total_local_spans(&self) -> usize {
        self.spans.read().unwrap().iter().filter(|v| v.1.is_local()).count()
    }

    pub fn total_remote_spans(&self) -> usize {
        self.spans.read().unwrap().len() - self.total_local_spans()
    }

    pub fn total_local_memory(&self) -> usize {
        self.spans.read().unwrap().iter().map(|v| v.1.local_memory_usage()).sum()
    }

    pub fn total_remote_memory(&self) -> usize {
        self.spans.read().unwrap().iter().map(|v| v.1.remote_memory_usage()).sum()
    }

    pub fn ensure_local_memory_under_limit(&self) {
        let current_local_memory = self.total_local_memory() as u64;
        if current_local_memory < self.local_memory_max_threshold {
            return;
        }

        let memory_to_swap_out = current_local_memory - self.local_memory_max_threshold;
        let mut spans_to_swap_out = Vec::new();
        
        let mut total_memory = 0;
        for (span_id, span) in self.spans.read().unwrap().iter() {
            if total_memory >= memory_to_swap_out {
                break;
            }

            if span.is_remote() {
                continue;
            }

            spans_to_swap_out.push(span_id.clone());
            total_memory += span.local_memory_usage() as u64;
        }

        self.swap_out_spans(&spans_to_swap_out);
    }
}

impl FarMemorySpan {
    pub fn is_local(&self) -> bool {
        match self {
            FarMemorySpan::Local { .. } => true,
            FarMemorySpan::Remote { .. } => false,
        }
    }

    pub fn is_remote(&self) -> bool {
        match self {
            FarMemorySpan::Local { .. } => false,
            FarMemorySpan::Remote { .. } => true,
        }
    }

    pub fn local_memory_usage(&self) -> usize {
        match self {
            FarMemorySpan::Local { ptr: _, size } => *size,
            FarMemorySpan::Remote { .. } => 0,
        }
    }

    pub fn remote_memory_usage(&self) -> usize {
        match self {
            FarMemorySpan::Local { .. } => 0,
            FarMemorySpan::Remote { size } => *size,
        } 
    }
}