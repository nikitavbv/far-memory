use {
    std::{sync::{Arc, atomic::{AtomicU64, Ordering}, RwLock}, collections::HashMap, alloc::{GlobalAlloc, Layout}},
    tracing::{Level, span},
    crate::utils::allocator::GLOBAL,
    super::{
        backend::FarMemoryBackend,
        span::{SpanId, FarMemorySpan},
    },
};

#[derive(Clone)]
pub struct FarMemoryClient {
    span_id_counter: Arc<AtomicU64>,
    spans: Arc<RwLock<HashMap<SpanId, FarMemorySpan>>>,

    backend: Arc<Box<dyn FarMemoryBackend>>,

    local_memory_max_threshold: u64,
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
        span!(Level::DEBUG, "allocate_span - ensure local memory limit").in_scope(|| {
            self.ensure_local_memory_under_limit(self.local_memory_max_threshold - span_size as u64);
        });

        let id = SpanId::from_id(self.span_id_counter.fetch_add(1, Ordering::Relaxed));
        self.spans.write().unwrap().insert(id.clone(), FarMemorySpan::new_local(span_size));
        id
    }

    pub fn span_ptr(&self, id: &SpanId) -> *mut u8 {
        let span_remote_size = {
            let span = &self.spans.read().unwrap()[id];
            if span.is_local() {
                return span.ptr();
            }
            
            // will need to swap in
            span.remote_memory_usage()
        };

        span!(Level::DEBUG, "span_ptr - ensure local memory limit").in_scope(|| {
            // only need to free as much memory as remote part will take. There is already memory for local part of span
            self.ensure_local_memory_under_limit(self.local_memory_max_threshold - span_remote_size as u64);
        });

        // swap in
        span!(Level::DEBUG, "span_ptr - swap_in", span_id = id.id(), span_remote_size).in_scope(|| {
            let data = span!(Level::DEBUG, "backend swap in").in_scope(|| self.backend.swap_in(id));
            let size = data.len();
    
            let ptr = unsafe {
                GLOBAL.alloc(self.span_layout(size))
            };
            unsafe {
                std::ptr::copy(data.as_slice() as *const _ as *const u8, ptr, size);
            }
    
            self.spans.write().unwrap().insert(id.clone(), FarMemorySpan::for_local_ptr_and_size(ptr.clone(), size));
    
            ptr
        })
    }

    fn span_layout(&self, span_size: usize) -> Layout {
        Layout::array::<u8>(span_size).unwrap()
    }

    pub fn swap_out_spans(&self, spans: &[SpanId]) {
        for span in spans {
            span!(Level::DEBUG, "swap out span", span_id = span.id()).in_scope(|| {
                self.swap_out_span(span)
            });
        }
    }

    fn swap_out_span(&self, span_id: &SpanId) {
        let span = self.spans.write().unwrap().remove(&span_id.clone()).unwrap();

        let local_span_data = match span {
            FarMemorySpan::Local(local) => local,
            FarMemorySpan::Remote { .. } => return,
        };

        let data = local_span_data.read_to_slice();

        span!(Level::DEBUG, "backend swap out", size = data.len()).in_scope(|| {
            self.backend.swap_out(span_id.clone(), data);
        });
        
        self.spans.write().unwrap().insert(span_id.clone(), FarMemorySpan::Remote { local_part: None, total_size: local_span_data.size() });

        local_span_data.free();
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

    fn ensure_local_memory_under_limit(&self, limit: u64) {
        let current_local_memory = self.total_local_memory() as u64;
        if current_local_memory < limit {
            return;
        }

        let memory_to_swap_out = current_local_memory - limit;
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

        span!(Level::DEBUG, "swap_out_spans", needed = memory_to_swap_out, swap_out_req_size = total_memory).in_scope(|| {
            self.swap_out_spans(&spans_to_swap_out);
        });
    }
}
