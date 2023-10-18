use {
    std::{sync::{Arc, atomic::{AtomicU64, Ordering}, RwLock}, collections::HashMap},
    tracing::{Level, span},
    super::{
        backend::FarMemoryBackend,
        span::{SpanId, FarMemorySpan, LocalSpanData},
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

        self.mark_span_in_use(id, true);
        span!(Level::DEBUG, "span_ptr - ensure local memory limit").in_scope(|| {
            // only need to free as much memory as remote part will take. There is already memory for local part of span
            self.ensure_local_memory_under_limit(self.local_memory_max_threshold - span_remote_size as u64);
        });

        // swap in
        span!(Level::DEBUG, "span_ptr - swap_in", span_id = id.id(), span_remote_size).in_scope(|| {
            let span = self.spans.write().unwrap().remove(id).unwrap();

            let data = span!(Level::DEBUG, "backend swap in").in_scope(|| self.backend.swap_in(id));
    
            // new swap in with support for partial
            let local_data = match span {
                FarMemorySpan::Local { .. } => panic!("didn't expect span that is being swapped in to be marked as local"),
                FarMemorySpan::Remote { local_part, total_size: _ } => local_part,
            };

            let local_data = if let Some(local_data) = local_data {
                local_data.extend_with_vec(data)
            } else {
                LocalSpanData::from_vec(data)
            };

            let ptr = local_data.ptr();        
            self.spans.write().unwrap().insert(id.clone(), FarMemorySpan::Local {
                data: local_data,
            });
            ptr
        })
    }

    pub fn swap_out_spans_fully(&self, spans: &[SpanId]) {
        for span in spans {
            self.swap_out_span(span, self.spans.read().unwrap().get(span).unwrap().local_memory_usage());
        }
    }

    pub fn swap_out_spans(&self, spans: &[(SpanId, usize)]) {
        // (span, how much memory to swap out - can be partial or full swap out)
        for (span, swap_out_size) in spans {
            span!(Level::DEBUG, "swap out span", span_id = span.id()).in_scope(|| {
                self.swap_out_span(span, *swap_out_size)
            });
        }
    }

    fn swap_out_span(&self, span_id: &SpanId, swap_out_size: usize) {
        let span = self.spans.write().unwrap().remove(&span_id.clone()).unwrap();

        let total_size = span.total_size();
        let (local_part, prepend_to_backend) = match span {
            FarMemorySpan::Local { data } => {
                if data.is_in_use() {
                    panic!("attempting to swap out span that is in use!");
                }
                (data, false) // not prepending to remote, because span is local
            },
            FarMemorySpan::Remote { local_part, total_size: _ } => (
                local_part.expect("expected span to contain local part when swapping out"), 
                true, // prepending, because this span already contains a remote part
            ),
        };
        if swap_out_size > local_part.size() {
            panic!("swap out size cannot be larger than local part size");
        }
        let remaining_local_part = local_part.size() - swap_out_size;
        let full_swap_out = remaining_local_part == 0;

        let data = if full_swap_out {
            local_part.read_to_slice()
        } else {
            // read from end
            local_part.read_to_slice_with_range(remaining_local_part..local_part.size())
        };

        span!(Level::DEBUG, "backend swap out", size = data.len()).in_scope(|| {
            self.backend.swap_out(span_id.clone(), data, prepend_to_backend);
        });

        if full_swap_out {
            self.spans.write().unwrap().insert(span_id.clone(), FarMemorySpan::Remote { local_part: None, total_size });
            local_part.free();
        } else {
            self.spans.write().unwrap().insert(span_id.clone(), FarMemorySpan::Remote { local_part: Some(local_part.shrink(swap_out_size)), total_size });
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

    fn ensure_local_memory_under_limit(&self, limit: u64) {
        let current_local_memory = self.total_local_memory() as u64;
        if current_local_memory < limit {
            return;
        }

        let memory_to_swap_out = current_local_memory - limit;
        let mut spans_to_swap_out = Vec::new(); // (span, how much memory to swap out - can be partial or full swap out)
        
        let mut total_memory = 0;
        for (span_id, span) in self.spans.read().unwrap().iter() {
            if total_memory >= memory_to_swap_out {
                break;
            }

            if span.is_in_use() {
                continue;
            }

            let span_local_memory_size = span.local_memory_usage();
            if span_local_memory_size == 0 {
                continue;
            }

            let span_swap_out_len = span_local_memory_size.min((memory_to_swap_out - total_memory) as usize);
            spans_to_swap_out.push((span_id.clone(), span_swap_out_len));
            total_memory += span_swap_out_len as u64;
        }

        span!(Level::DEBUG, "swap_out_spans", needed = memory_to_swap_out, swap_out_req_size = total_memory).in_scope(|| {
            self.swap_out_spans(&spans_to_swap_out);
        });
    }

    pub fn mark_span_in_use(&self, id: &SpanId, in_use: bool) {
        self.spans.write().unwrap().get_mut(id).unwrap().mark_in_use(in_use);
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::client::InMemoryBackend,
        super::*,
    };

    #[test]
    fn partial_swap_out() {
        let client = FarMemoryClient::new(Box::new(InMemoryBackend::new()), 30);
        let span = client.allocate_span(20);

        assert_eq!(20, client.total_local_memory());
        assert_eq!(0, client.total_remote_memory());

        client.ensure_local_memory_under_limit(15);
        assert_eq!(15, client.total_local_memory());
        assert_eq!(5, client.total_remote_memory());

        let _ptr = client.span_ptr(&span);
        assert_eq!(20, client.total_local_memory());
        assert_eq!(0, client.total_remote_memory());
    }
}