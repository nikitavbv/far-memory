use {
    std::{sync::{Arc, atomic::{AtomicU64, Ordering, AtomicBool}, RwLock, Mutex}, collections::HashMap, thread, time::Duration},
    tracing::{Level, span, info},
    crossbeam::utils::Backoff,
    prometheus::Registry,
    super::{
        backend::FarMemoryBackend,
        prefetching::{EvictionPolicy, MostRecentlyUsedEvictionPolicy, PreferRemoteSpansEvictionPolicy, ReplayEvictionPolicy},
        span::{SpanId, FarMemorySpan, LocalSpanData},
    },
};

#[derive(Clone)]
pub struct FarMemoryClient {
    span_id_counter: Arc<AtomicU64>,
    spans: Arc<RwLock<HashMap<SpanId, FarMemorySpan>>>,
    is_running: Arc<AtomicBool>,

    backend: Arc<Box<dyn FarMemoryBackend>>,
    eviction_policy: Arc<Box<dyn EvictionPolicy>>,

    local_memory_max_threshold: u64,

    swap_in_out_lock: Arc<Mutex<()>>,
    span_states: Arc<RwLock<HashMap<SpanId, Mutex<SpanState>>>>,
}

#[derive(Eq, PartialEq)]
enum SpanState {
    Free,
    InUse(usize),
    SwappingOut,
}

impl FarMemoryClient {
    pub fn new(backend: Box<dyn FarMemoryBackend>, local_memory_max_threshold: u64) -> Self {
        Self {
            span_id_counter: Arc::new(AtomicU64::new(0)),
            spans: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(AtomicBool::new(true)),

            backend: Arc::new(backend),
            eviction_policy: Arc::new(Box::new(ReplayEvictionPolicy::new(Box::new(PreferRemoteSpansEvictionPolicy::new(Box::new(MostRecentlyUsedEvictionPolicy::new())))))),
            local_memory_max_threshold,

            swap_in_out_lock: Arc::new(Mutex::new(())),
            span_states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn track_metrics(&self, registry: Registry) {
    }

    pub fn start_swap_out_thread(&self) {
        thread::Builder::new().name("swap-out".to_owned())
            .spawn(swap_out_thread(
                self.clone(),
                self.local_memory_max_threshold - 10 * 1024 * 1024
            )).unwrap();
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
        self.eviction_policy.on_stop();
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn allocate_span(&self, span_size: usize) -> SpanId {
        let _guard = span!(Level::DEBUG, "waiting for lock").in_scope(|| self.swap_in_out_lock.lock().unwrap());

        span!(Level::DEBUG, "allocate_span - ensure local memory limit").in_scope(|| {
            self.ensure_local_memory_under_limit(self.local_memory_max_threshold - span_size as u64);
        });

        let id = SpanId::from_id(self.span_id_counter.fetch_add(1, Ordering::Relaxed));
        self.spans.write().unwrap().insert(id.clone(), FarMemorySpan::new_local(span_size));
        self.span_states.write().unwrap().insert(id.clone(), Mutex::new(SpanState::Free));
        id
    }

    pub fn span_ptr(&self, id: &SpanId) -> *mut u8 {
        self.eviction_policy.on_span_access(id);

        let span_remote_size = {
            let backoff = Backoff::new();
            loop {
                let span_states = self.span_states.read().unwrap();
                let mut span_state = span_states[id].lock().unwrap();
                match &*span_state {
                    SpanState::Free => {
                        *span_state = SpanState::InUse(1);
                        break;
                    },
                    SpanState::InUse(refs) => {
                        *span_state = SpanState::InUse(refs + 1);
                        break;
                    },
                    SpanState::SwappingOut => {
                        // waiting for swap out to finish to swap back in again
                        backoff.spin();
                    },
                };
            }

            let span = &self.spans.read().unwrap()[id];
            if span.is_local() {
                return span.ptr();
            }

            // will need to swap in
            span.remote_memory_usage()
        };

        let _guard = span!(Level::DEBUG, "waiting for lock").in_scope(|| self.swap_in_out_lock.lock().unwrap());

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

            self.eviction_policy.on_span_swap_in(id);

            ptr
        })
    }

    pub fn span_local_memory_usage(&self, span_id: &SpanId) -> usize {
        self.spans.read().unwrap().get(&span_id).unwrap().local_memory_usage()
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

        let span_states = self.span_states.read().unwrap();
        let mut span_state = span_states[&span_id].lock().unwrap();
        if *span_state != SpanState::SwappingOut {
            panic!("expected span to be in swapping out state when actually swapping out");
        }
        *span_state = SpanState::Free;
        self.eviction_policy.on_span_swap_out(span_id);
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
        let mut possible_swap_out_spans: Vec<SpanId> = self.spans.read().unwrap().keys().cloned().collect();

        while !possible_swap_out_spans.is_empty() {
            if total_memory >= memory_to_swap_out {
                break;
            }

            let span_id = span!(Level::DEBUG, "picking span for eviction").in_scope(|| self.eviction_policy.pick_for_eviction(&possible_swap_out_spans).clone());
            let index = possible_swap_out_spans.iter().position(|x| *x == span_id).unwrap();
            possible_swap_out_spans.remove(index);

            let spans = self.spans.read().unwrap();
            let span = spans.get(&span_id).unwrap();
            {
                let span_states = self.span_states.read().unwrap();
                let mut span_state = span_states[&span_id].lock().unwrap();
                match &*span_state {
                    SpanState::Free => {
                        let span_local_memory_size = span.local_memory_usage();
                        if span_local_memory_size == 0 {
                            continue;
                        }

                        *span_state = SpanState::SwappingOut;

                        let span_swap_out_len = span_local_memory_size.min((memory_to_swap_out - total_memory) as usize);
                        spans_to_swap_out.push((span_id.clone(), span_swap_out_len));
                        total_memory += span_swap_out_len as u64;
                    },
                    SpanState::InUse(_) => continue, // cannot swap out span that is in use
                    SpanState::SwappingOut => continue, // cannot swap out span that is already being swapped out
                }
            }
        }

        span!(Level::DEBUG, "swap_out_spans", needed = memory_to_swap_out, swap_out_req_size = total_memory).in_scope(|| {
            self.swap_out_spans(&spans_to_swap_out);
        });
    }

    pub fn decrease_refs_for_span(&self, span_id: &SpanId) {
        let span_states = self.span_states.read().unwrap();
        let mut span_state = span_states[span_id].lock().unwrap();
        match &*span_state {
            SpanState::Free => panic!("span is already free!"),
            SpanState::InUse(refs) => *span_state = if *refs == 1 {
                SpanState::Free
            } else {
                SpanState::InUse(refs - 1)
            },
            SpanState::SwappingOut => panic!("cannot decrease refs for span that is being swapped out")
        }
    }
}

fn swap_out_thread(client: FarMemoryClient, target_memory_usage: u64) -> impl FnOnce() -> () {
    move || {
        info!("starting swap out thread");
        span!(Level::DEBUG, "swap out thread").in_scope(|| {
            while client.is_running() {
                thread::sleep(Duration::from_secs(10));

                span!(Level::DEBUG, "swap out iteration").in_scope(|| {
                    let _guard = span!(Level::DEBUG, "waiting for lock").in_scope(|| client.swap_in_out_lock.lock().unwrap());
                    client.ensure_local_memory_under_limit(target_memory_usage);
                });
            }
        });
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

    #[test]
    fn partial_swap_out_multiple_parts() {
        let client = FarMemoryClient::new(Box::new(InMemoryBackend::new()), 30);
        let span = client.allocate_span(20);

        client.ensure_local_memory_under_limit(15);
        assert_eq!(15, client.total_local_memory());
        assert_eq!(5, client.total_remote_memory());

        client.ensure_local_memory_under_limit(10);
        assert_eq!(10, client.total_local_memory());
        assert_eq!(10, client.total_remote_memory());

        let _ptr = client.span_ptr(&span);
        assert_eq!(20, client.total_local_memory()); // first part (5) and second (5) were both swapped, so +10.
        assert_eq!(0, client.total_remote_memory());
    }
}
