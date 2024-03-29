use {
    std::{sync::{Arc, atomic::{AtomicU64, Ordering, AtomicBool}, RwLock, Mutex}, collections::HashMap, thread, time::{Instant, Duration}},
    tracing::{Level, span, info, debug, warn},
    crossbeam::utils::Backoff,
    prometheus::{Registry, register_int_gauge_with_registry, IntGauge, IntCounter, register_int_counter_with_registry},
    crate::manager::ManagerClient,
    super::{
        backend::{FarMemoryBackend, SwapOutOperation, SwapOutOperationData},
        replacement::{ReplacementPolicy, MostRecentlyUsedReplacementPolicy, PreferRemoteSpansReplacementPolicy, ReplayReplacementPolicy},
        span::{SpanId, FarMemorySpan, LocalSpanData},
        object::{ObjectId, ObjectRegistry, ObjectLocation, FarMemory},
        serialized_object_vec::FarMemorySerializedObjectVec,
        vec::FarMemoryVec,
    },
};

#[derive(Clone)]
pub struct FarMemoryClient {
    span_id_counter: Arc<AtomicU64>,
    spans: Arc<RwLock<HashMap<SpanId, FarMemorySpan>>>,
    is_running: Arc<AtomicBool>,

    backend: Arc<Box<dyn FarMemoryBackend>>,
    replacement_policy: Arc<Box<dyn ReplacementPolicy>>,
    manager: Arc<Option<ManagerClient>>,

    local_memory_max_threshold: u64,
    swap_out_min_size: Option<u64>,

    swap_in_out_lock: Arc<Mutex<()>>,
    span_states: Arc<RwLock<HashMap<SpanId, Mutex<SpanState>>>>,

    object_registry: Arc<ObjectRegistry>,

    metrics: Option<ClientMetrics>,
}

#[derive(Eq, PartialEq)]
enum SpanState {
    Free, // can be local or remote
    InUse(usize), // span is in use, it is local
    Swapping, // swapping in or swapping out
}

struct SwapOutResult {
    spans: usize,
    bytes: usize,

    swap_in_span_data: Option<Vec<u8>>, // data of a span that was swapped in during the same request
}

impl FarMemoryClient {
    // higher level API
    pub fn connect_to(manager_endpoint: &str, token: &str) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn object<T>(&self, object: T) -> FarMemory<T> {
        unimplemented!()
    }

    pub fn vec<T>(&self, data: Vec<T>) -> FarMemoryVec<T> {
        FarMemoryVec::from_vec(self.clone(), data)
    }

    pub fn serialized_object_vec<T>(&self, objects: Vec<T>) -> FarMemorySerializedObjectVec<T> {
        unimplemented!()
    }

    // lower level API
    pub fn new(backend: Box<dyn FarMemoryBackend>, local_memory_max_threshold: u64) -> Self {
        Self {
            span_id_counter: Arc::new(AtomicU64::new(0)),
            spans: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(AtomicBool::new(true)),

            backend: Arc::new(backend),
            replacement_policy: Arc::new(Box::new(ReplayReplacementPolicy::new(Box::new(PreferRemoteSpansReplacementPolicy::new(Box::new(MostRecentlyUsedReplacementPolicy::new())))))),
            manager: Arc::new(None),
            local_memory_max_threshold,
            swap_out_min_size: None,

            swap_in_out_lock: Arc::new(Mutex::new(())),
            span_states: Arc::new(RwLock::new(HashMap::new())),

            object_registry: Arc::new(ObjectRegistry::new()),

            metrics: None,
        }
    }

    pub fn use_manager(&mut self, manager: ManagerClient) {
        self.manager = Arc::new(Some(manager));
    }

    pub fn use_replacement_policy(&mut self, replacement_policy: Box<dyn ReplacementPolicy>) {
        self.replacement_policy = Arc::new(replacement_policy);
    }

    pub fn track_metrics(&mut self, registry: Registry) {
        self.metrics = Some(ClientMetrics::new(registry));
        self.start_metrics_thread();
    }

    pub fn start_swap_out_thread(&self) {
        thread::Builder::new().name("swap-out".to_owned())
            .spawn(swap_out_thread(
                self.clone(),
                self.local_memory_max_threshold - 256 * 1024 * 1024
            )).unwrap();
    }

    pub fn start_metrics_thread(&self) {
        thread::Builder::new().name("metrics".to_owned())
            .spawn(report_metrics_thread(self.clone()))
            .unwrap();
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
        self.replacement_policy.on_stop();
        self.backend.on_stop();
        if let Some(metrics) = self.metrics.as_ref() {
            metrics.unregister();
        }
        if let Some(manager) = self.manager.as_ref() {
            manager.on_stop();
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    pub fn set_swap_out_min_size(&mut self, swap_out_min_size: u64) {
        self.swap_out_min_size = Some(swap_out_min_size);
    }

    pub fn allocate_span(&self, span_size: usize) -> SpanId {
        span!(Level::DEBUG, "allocate_span - ensure local memory limit").in_scope(|| {
            self.ensure_local_memory_under_limit(self.local_memory_max_threshold - span_size as u64, true);
        });

        let _guard = span!(Level::DEBUG, "waiting for lock").in_scope(|| self.swap_in_out_lock.lock().unwrap());
        let id = SpanId::from_id(self.span_id_counter.fetch_add(1, Ordering::Relaxed));
        self.replacement_policy.on_new_span(&id);
        self.spans.write().unwrap().insert(id.clone(), FarMemorySpan::new_local(span_size));
        self.span_states.write().unwrap().insert(id.clone(), Mutex::new(SpanState::Free));
        id
    }

    pub fn span_ptr(&self, id: &SpanId) -> *mut u8 {
        let started_at = Instant::now();

        self.replacement_policy.on_span_access(id);
        if let Some(metrics) = self.metrics.as_ref() {
            metrics.span_access_ops.inc();
        }

        let span_remote_size = {
            let backoff = Backoff::new();
            let waiting_for_span_lock = span!(Level::DEBUG, "waiting for span lock");
            let mut waiting_for_span_lock_guard = None;
            loop {
                let span_states = self.span_states.read().unwrap();
                let mut span_state = span_states[id].lock().unwrap();
                match &*span_state {
                    SpanState::Free => {
                        drop(waiting_for_span_lock_guard);

                        let span = &self.spans.read().unwrap()[id];
                        if span.is_local() {
                            // span is local already, so no need to swap it in
                            // marking it as in use:
                            *span_state = SpanState::InUse(1);

                            if let Some(metrics) = &self.metrics {
                                metrics.access_latency_micros_local.inc_by((Instant::now() - started_at).as_micros() as u64);
                            }
                            return span.ptr();
                        } else {
                            // span is not local, so will need to swap it in
                            // marking it as in swapping state
                            *span_state = SpanState::Swapping;
                            break span.remote_memory_usage();
                        };
                    },
                    SpanState::InUse(refs) => {
                        drop(waiting_for_span_lock_guard);

                        *span_state = SpanState::InUse(refs + 1);

                        let span = &self.spans.read().unwrap()[id];
                        if let Some(metrics) = &self.metrics {
                            metrics.access_latency_micros_local.inc_by((Instant::now() - started_at).as_micros() as u64);
                        }
                        return span.ptr();
                    },
                    SpanState::Swapping => {
                        // waiting for swap out to finish to swap back in again
                        // or waiting for it to finish swapping in
                        if waiting_for_span_lock_guard.is_none() {
                            waiting_for_span_lock_guard = Some(waiting_for_span_lock.enter());
                        }
                        backoff.spin();
                    },
                };
            }
        };

        let data = span!(Level::DEBUG, "swap out and swap in").in_scope(|| {
            // only need to free as much memory as remote part will take. There is already memory for local part of span
            let result = self.ensure_local_memory_under_limit_and_swap_in(self.local_memory_max_threshold - span_remote_size as u64, Some(id), true);
            if let Some(metrics) = &self.metrics {
                metrics.span_swap_out_on_access_ops.inc_by(result.spans as u64);
            }
            result.swap_in_span_data.unwrap()
        });

        // swap in
        span!(Level::DEBUG, "span_ptr - swap_in", span_id = id.id(), span_remote_size).in_scope(|| {
            let span = self.spans.write().unwrap().remove(id).unwrap();

            // new swap in with support for partial
            let local_data = match span {
                FarMemorySpan::Local { .. } => panic!("didn't expect span that is being swapped in to be marked as local"),
                FarMemorySpan::Remote { local_part, total_size: _ } => local_part,
            };

            let local_data = span!(Level::DEBUG, "creating local data").in_scope(|| if let Some(local_data) = local_data {
                span!(Level::DEBUG, "extending local data").in_scope(|| local_data.extend_with_vec(data))
            } else {
                span!(Level::DEBUG, "creating local data from vec").in_scope(|| LocalSpanData::from_vec(data))
            });

            let ptr = local_data.ptr();
            self.spans.write().unwrap().insert(id.clone(), FarMemorySpan::Local {
                data: local_data,
            });

            {
                let span_states = self.span_states.read().unwrap();
                let mut span_state = span_states[id].lock().unwrap();
                match &*span_state {
                    SpanState::Free => panic!("did not expect span state to be free when finishing swapping in"),
                    SpanState::InUse(_) => panic!("did not expect span state to be in use when finishing swapping in"),
                    SpanState::Swapping => *span_state = SpanState::InUse(1),
                };
            }

            self.replacement_policy.on_span_swap_in(id);
            if let Some(metrics) = self.metrics.as_ref() {
                metrics.span_swap_in_ops.inc();
                metrics.access_latency_micros_swap_in.inc_by((Instant::now() - started_at).as_micros() as u64);
            }

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
        self.swap_out_spans_and_swap_in(spans, None);
    }

    fn swap_out_spans_and_swap_in(&self, spans: &[(SpanId, usize)], swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        struct SwapOutFinalizeOperation {
            span_id: SpanId,
            local_part: Option<LocalSpanData>,
            full_swap_out: bool,
            total_size: usize,
            swap_out_size: usize,
        }

        let mut swap_out_ops = Vec::new();
        let mut finalize_ops: Vec<SwapOutFinalizeOperation> = Vec::new();

        // (span, how much memory to swap out - can be partial or full swap out)
        for (span_id, swap_out_size) in spans {
            span!(Level::DEBUG, "creating swap op").in_scope(|| {
                let span = self.spans.write().unwrap().remove(&span_id).unwrap();
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
                if *swap_out_size > local_part.size() {
                    panic!("swap out size cannot be larger than local part size");
                }
                let remaining_local_part = local_part.size() - swap_out_size;
                let full_swap_out = remaining_local_part == 0;

                let (data, local_part) = span!(Level::DEBUG, "reading local part").in_scope(|| if full_swap_out {
                    (SwapOutOperationData::Owned(local_part.into_vec()), None)
                } else {
                    // read from end
                    (local_part.to_swap_out_operation_data_with_range(remaining_local_part..local_part.size()), Some(local_part))
                });

                let push_ops_span = span!(Level::DEBUG, "push ops");
                let _push_ops_span_guard = push_ops_span.enter();
                swap_out_ops.push(SwapOutOperation::new(span_id.clone(), span!(Level::DEBUG, "data to vec", full_swap_out).in_scope(|| data), prepend_to_backend));
                finalize_ops.push(SwapOutFinalizeOperation { span_id: span_id.clone(), local_part, full_swap_out, total_size, swap_out_size: *swap_out_size })
            });
        }

        let swap_in_data = span!(Level::DEBUG, "backend batch swap").in_scope(|| {
            self.backend.batch(swap_out_ops, swap_in)
        });

        span!(Level::DEBUG, "swap out finalize ops").in_scope(|| {
            for op in finalize_ops {
                span!(Level::DEBUG, "finalize op").in_scope(|| {
                    if op.full_swap_out {
                        self.spans.write().unwrap().insert(op.span_id.clone(), FarMemorySpan::Remote { local_part: None, total_size: op.total_size });
                    } else {
                        self.spans.write().unwrap().insert(
                            op.span_id.clone(),
                            FarMemorySpan::Remote {
                                local_part: Some(span!(Level::DEBUG, "shrinking local part").in_scope(|| op.local_part.unwrap().shrink(op.swap_out_size))),
                                total_size: op.total_size
                            }
                        );
                    }

                    let span_states = self.span_states.read().unwrap();
                    let mut span_state = span_states[&op.span_id].lock().unwrap();
                    if *span_state != SpanState::Swapping {
                        panic!("expected span to be in swapping state when actually swapping out");
                    }
                    *span_state = SpanState::Free;
                    self.replacement_policy.on_span_swap_out(&op.span_id, !op.full_swap_out);

                    if let Some(metrics) = self.metrics.as_ref() {
                        metrics.span_swap_out_ops.inc();
                    }
                });
            }
        });

        swap_in_data
    }

    fn swap_out_span(&self, span_id: &SpanId, swap_out_size: usize) {
        let span = self.spans.write().unwrap().remove(&span_id).unwrap();

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
        if *span_state != SpanState::Swapping {
            panic!("expected span to be in swapping out state when actually swapping out");
        }
        *span_state = SpanState::Free;
        self.replacement_policy.on_span_swap_out(span_id, !full_swap_out);

        if let Some(metrics) = self.metrics.as_ref() {
            metrics.span_swap_out_ops.inc();
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

    fn ensure_local_memory_under_limit(&self, limit: u64, strict: bool) -> SwapOutResult {
        self.ensure_local_memory_under_limit_and_swap_in(limit, None, strict)
    }

    /// strict: whether to wait if there are no enough spans to swap out to fulfill memory limit request
    fn ensure_local_memory_under_limit_and_swap_in(&self, limit: u64, swap_in: Option<&SpanId>, strict: bool) -> SwapOutResult {
        let current_local_memory = self.total_local_memory() as u64;
        if current_local_memory < limit {
            return SwapOutResult {
                spans: 0,
                bytes: 0,
                swap_in_span_data: swap_in.map(|v| self.backend.swap_in(v)),
            };
        }

        let _swap_ops_lock_guard = span!(Level::DEBUG, "waiting for lock").in_scope(|| self.swap_in_out_lock.lock().unwrap());
        let memory_to_swap_out = current_local_memory - limit;
        let memory_to_swap_out = if let Some(min_size) = self.swap_out_min_size {
            memory_to_swap_out.max(min_size)
        } else {
            memory_to_swap_out
        };

        let mut spans_to_swap_out = Vec::new(); // (span, how much memory to swap out - can be partial or full swap out)

        let mut total_memory = 0;
        let possible_swap_out_spans: Vec<SpanId> = self.spans.read().unwrap().keys().cloned().collect(); // TODO: remove this completely

        let mut spans_for_eviction = span!(Level::DEBUG, "querying replacement policy").in_scope(|| self.replacement_policy.pick_for_eviction(&possible_swap_out_spans));

        span!(Level::DEBUG, "picking spans for eviction", total_spans=possible_swap_out_spans.len()).in_scope(|| {
            'spans_picking: loop {
                if total_memory >= memory_to_swap_out {
                    break;
                }

                let span_id = loop {
                    if let Some(span_id) = spans_for_eviction.next() {
                        break span_id;
                    } else {
                        warn!("there are no spans to evict remaining that can be picked");
                        if strict {
                            spans_for_eviction = span!(Level::DEBUG, "querying replacement policy").in_scope(|| self.replacement_policy.pick_for_eviction(&possible_swap_out_spans));
                            continue;
                        } else {
                            break 'spans_picking;
                        }
                    }
                };

                {
                    let span_states = self.span_states.read().unwrap();
                    let mut span_state = span_states[&span_id].lock().unwrap();
                    match &*span_state {
                        SpanState::Free => {
                            let spans = self.spans.read().unwrap();
                            let span = spans.get(&span_id).unwrap();
                            let span_local_memory_size = span.local_memory_usage();
                            if span_local_memory_size == 0 {
                                debug!(span_id=span_id.id(), "skipping span that does not have local memory");
                                continue;
                            }

                            // marking swap as in swapping state so anyone else who needs it has to wait until it is fully swapped out.
                            *span_state = SpanState::Swapping;

                            let span_swap_out_len = span_local_memory_size.min((memory_to_swap_out - total_memory) as usize);
                            spans_to_swap_out.push((span_id.clone(), span_swap_out_len));
                            total_memory += span_swap_out_len as u64;
                        },
                        SpanState::InUse(_) => {
                            // cannot swap out span that is in use
                            debug!(span_id=span_id.id(), "skipping span that is in use");
                            continue;
                        },
                        SpanState::Swapping => {
                            // cannot swap out span that is already being swapped out or is in progress of being swapped in
                            debug!(span_id=span_id.id(), "skipping span that is in process of swapping");
                            continue;
                        },
                    }
                }
            }
        });

        let swap_in_span_data = span!(Level::DEBUG, "perform swapping", needed = memory_to_swap_out, swap_out_req_size = total_memory).in_scope(|| {
            self.swap_out_spans_and_swap_in(&spans_to_swap_out, swap_in)
        });

        drop(_swap_ops_lock_guard);

        SwapOutResult {
            spans: spans_to_swap_out.len(),
            bytes: total_memory as usize,
            swap_in_span_data,
        }
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
            SpanState::Swapping => panic!("cannot decrease refs for span that is being swapped out or swapped in")
        }
    }

    // objects
    pub fn put_object(&self, object: Vec<u8>) -> ObjectId {
        let object_id = self.object_registry.next_object_id();
        let object_location = self.object_registry.put_object(object_id.clone(), object.len());
        let object_location = if let Some(object_location) = object_location {
            // append object to existing span
            object_location
        } else {
            // create new span for this object
            let size_class = self.object_registry.size_class_for_object(object.len());
            let span_size = 2 * 1024 * 1024;
            let span_size = span_size + (size_class - span_size % size_class);
            let span = self.allocate_span(span_size);
            self.object_registry.add_span_for_object(span.clone(), span_size, object_id.clone(), object.len())
        };

        let span_ptr = self.span_ptr(&object_location.span_id);
        unsafe {
            std::ptr::copy_nonoverlapping(object.as_ptr(), span_ptr.add(object_location.offset), object.len());
        }
        self.decrease_refs_for_span(&object_location.span_id);

        object_id
    }

    pub fn get_object(&self, object_id: &ObjectId) -> ObjectLocation {
        self.object_registry.get_object(object_id)
    }

    pub fn is_object_local(&self, object_id: &ObjectId) -> bool {
        let location = self.object_registry.get_object(object_id);
        self.is_span_local(&location.span_id)
    }

    fn is_span_local(&self, span_id: &SpanId) -> bool {
        self.spans.read().unwrap().get(span_id).map(|v| v.is_local()).unwrap_or(false)
    }
}

#[derive(Clone)]
struct ClientMetrics {
    registry: Registry,

    local_memory: IntGauge,
    remote_memory: IntGauge,
    local_spans: IntGauge,
    remote_spans: IntGauge,

    span_access_ops: IntCounter,
    span_swap_in_ops: IntCounter,
    span_swap_out_ops: IntCounter,
    span_swap_out_on_access_ops: IntCounter,

    background_swap_out_spans: IntCounter,
    background_swap_out_bytes: IntCounter,

    access_latency_micros_local: IntCounter,
    access_latency_micros_swap_in: IntCounter,
}

impl ClientMetrics {
    pub fn new(registry: Registry) -> Self {
        Self {
            registry: registry.clone(),

            local_memory: register_int_gauge_with_registry!(
                "client_local_memory",
                "local memory in bytes",
                registry
            ).unwrap(),
            remote_memory: register_int_gauge_with_registry!(
                "client_remote_memory",
                "remote memory in bytes",
                registry
            ).unwrap(),
            local_spans: register_int_gauge_with_registry!(
                "client_local_spans",
                "number of local spans",
                registry
            ).unwrap(),
            remote_spans: register_int_gauge_with_registry!(
                "client_remote_spans",
                "number of remote spans",
                registry
            ).unwrap(),

            span_access_ops: register_int_counter_with_registry!(
                "client_span_access_ops",
                "total span access operations",
                registry
            ).unwrap(),
            span_swap_in_ops: register_int_counter_with_registry!(
                "client_swap_in_ops",
                "total span swap in operations",
                registry
            ).unwrap(),
            span_swap_out_ops: register_int_counter_with_registry!(
                "client_swap_out_ops",
                "total span swap out operations",
                registry
            ).unwrap(),
            span_swap_out_on_access_ops: register_int_counter_with_registry!(
                "client_swap_out_on_access_ops",
                "total swap out ops to free memory when accessing span",
                registry
            ).unwrap(),

            background_swap_out_spans: register_int_counter_with_registry!(
                "client_background_swap_out_spans",
                "swapped out spans by background thread",
                registry
            ).unwrap(),
            background_swap_out_bytes: register_int_counter_with_registry!(
                "client_background_swap_out_bytes",
                "swapped out bytes by background thread",
                registry
            ).unwrap(),

            access_latency_micros_local: register_int_counter_with_registry!(
                "client_access_latency_local",
                "local span access latency in microseconds",
                registry
            ).unwrap(),
            access_latency_micros_swap_in: register_int_counter_with_registry!(
                "client_access_latency_swap_in",
                "remote span access latency in microseconds",
                registry
            ).unwrap(),
        }
    }

    pub fn unregister(&self) {
        self.registry.unregister(Box::new(self.local_memory.clone())).unwrap();
        self.registry.unregister(Box::new(self.remote_memory.clone())).unwrap();
        self.registry.unregister(Box::new(self.local_spans.clone())).unwrap();
        self.registry.unregister(Box::new(self.remote_spans.clone())).unwrap();

        self.registry.unregister(Box::new(self.span_access_ops.clone())).unwrap();
        self.registry.unregister(Box::new(self.span_swap_in_ops.clone())).unwrap();
        self.registry.unregister(Box::new(self.span_swap_out_ops.clone())).unwrap();
        self.registry.unregister(Box::new(self.span_swap_out_on_access_ops.clone())).unwrap();

        self.registry.unregister(Box::new(self.background_swap_out_spans.clone())).unwrap();
        self.registry.unregister(Box::new(self.background_swap_out_bytes.clone())).unwrap();

        self.registry.unregister(Box::new(self.access_latency_micros_local.clone())).unwrap();
        self.registry.unregister(Box::new(self.access_latency_micros_swap_in.clone())).unwrap();
    }
}

fn swap_out_thread(client: FarMemoryClient, target_memory_usage: u64) -> impl FnOnce() -> () {
    move || {
        info!("starting swap out thread");
        span!(Level::DEBUG, "swap out thread").in_scope(|| {
            while client.is_running() {
                thread::sleep(Duration::from_millis(16));

                let swap_out_result = span!(Level::DEBUG, "swap out iteration").in_scope(|| {
                    client.ensure_local_memory_under_limit(target_memory_usage, false)
                });

                if let Some(metrics) = client.metrics.as_ref() {
                    metrics.background_swap_out_spans.inc_by(swap_out_result.spans as u64);
                    metrics.background_swap_out_bytes.inc_by(swap_out_result.bytes as u64);
                }
            }
        });
    }
}

fn report_metrics_thread(client: FarMemoryClient) -> impl FnOnce() -> () {
    move || {
        while client.is_running() {
            let metrics = client.metrics.as_ref().unwrap();
            metrics.local_memory.set(client.total_local_memory() as i64);
            metrics.remote_memory.set(client.total_remote_memory() as i64);
            metrics.local_spans.set(client.total_local_spans() as i64);
            metrics.remote_spans.set(client.total_remote_spans() as i64);

            thread::sleep(Duration::from_secs(10));
        }
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

        client.ensure_local_memory_under_limit(15, true);
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

        client.ensure_local_memory_under_limit(15, true);
        assert_eq!(15, client.total_local_memory());
        assert_eq!(5, client.total_remote_memory());

        client.ensure_local_memory_under_limit(10, true);
        assert_eq!(10, client.total_local_memory());
        assert_eq!(10, client.total_remote_memory());

        let _ptr = client.span_ptr(&span);
        assert_eq!(20, client.total_local_memory()); // first part (5) and second (5) were both swapped, so +10.
        assert_eq!(0, client.total_remote_memory());
    }
}
