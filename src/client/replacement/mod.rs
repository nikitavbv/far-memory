use {
    std::{sync::{atomic::{AtomicU64, Ordering}, RwLock}, collections::{HashMap, HashSet}, path::Path, fs},
    rand::seq::SliceRandom,
    super::SpanId,
};

pub use {
    tracking::TrackingReplacementPolicy,
    replay::RemoteReplayReplacementPolicy,
};

mod tracking;
mod replay;

pub trait ReplacementPolicy: Send + Sync {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId;

    fn on_span_access(&self, span_id: &SpanId) {}
    fn on_span_swap_out(&self, span_id: &SpanId) {}
    fn on_span_swap_in(&self, span_id: &SpanId) {}
    fn on_stop(&self) {}
}

// 6.01 per token (for 25700)
pub struct RandomReplacementPolicy {
}

impl RandomReplacementPolicy {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl ReplacementPolicy for RandomReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a [SpanId]) -> &'a SpanId {
        spans.choose(&mut rand::thread_rng()).unwrap()
    }
}

// 108.3 per token (for 25700)
pub struct LeastRecentlyUsedReplacementPolicy {
    counter: AtomicU64,
    history: RwLock<HashMap<SpanId, u64>>,
}

impl LeastRecentlyUsedReplacementPolicy {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            history: RwLock::new(HashMap::new()),
        }
    }
}

impl ReplacementPolicy for LeastRecentlyUsedReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        let history = self.history.read().unwrap();
        spans.iter().map(|v| (v, history.get(v).unwrap_or(&0))).reduce(|a, b| if a.1 < b.1 { a } else { b }).map(|a| a.0).unwrap()
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.history.write().unwrap().insert(span_id.clone(), self.counter.fetch_add(1, Ordering::Relaxed));;
    }
}

// 5.42 per token (for 25700)
// 12.31 per token (for 25600)
pub struct MostRecentlyUsedReplacementPolicy {
    counter: AtomicU64,
    history: RwLock<HashMap<SpanId, u64>>,
}

impl MostRecentlyUsedReplacementPolicy {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            history: RwLock::new(HashMap::new()),
        }
    }
}

impl ReplacementPolicy for MostRecentlyUsedReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        let history = self.history.read().unwrap();
        spans.iter()
            .map(|v| (v, history.get(v).unwrap_or(&0)))
            .reduce(|a, b| if a.1 > b.1 { a } else { b })
            .map(|a| a.0)
            .unwrap()
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.history.write().unwrap().insert(span_id.clone(), self.counter.fetch_add(1, Ordering::Relaxed));;
    }
}

// current best when combined with MostRecentlyUsedReplacementPolicy
// 8.34 per token (for 25600)
pub struct PreferRemoteSpansReplacementPolicy {
    remote_spans: RwLock<HashSet<SpanId>>,
    inner: Box<dyn ReplacementPolicy>,
}

impl PreferRemoteSpansReplacementPolicy {
    pub fn new(inner: Box<dyn ReplacementPolicy>) -> Self {
        Self {
            remote_spans: RwLock::new(HashSet::new()),
            inner,
        }
    }
}

impl ReplacementPolicy for PreferRemoteSpansReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        let remote_spans: Vec<_> = {
            let remote_spans = self.remote_spans.try_read().unwrap();
            spans.iter().filter(|s| remote_spans.contains(s)).cloned().collect()
        };

        if !remote_spans.is_empty() {
            let span = self.inner.pick_for_eviction(&remote_spans);
            spans.iter().find(|v| *v == span).unwrap()
        } else {
            self.inner.pick_for_eviction(spans)
        }
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.inner.on_span_access(span_id)
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.inner.on_span_swap_in(span_id);
        self.remote_spans.write().unwrap().remove(span_id);
    }

    fn on_span_swap_out(&self, span_id: &SpanId) {
        self.inner.on_span_swap_out(span_id);
        self.remote_spans.write().unwrap().insert(span_id.clone());
    }
}

pub struct ReplayReplacementPolicy {
    history_file_path: String,
    record_mode: bool,
    history: RwLock<Vec<SpanId>>,
    access_counter: AtomicU64,
    fallback: Box<dyn ReplacementPolicy>,
}

impl ReplayReplacementPolicy {
    pub fn new(fallback: Box<dyn ReplacementPolicy>) -> Self {
        let history_file_path = "./data/eviction_history.json".to_owned();

        let path = Path::new(&history_file_path);
        let record_mode = !path.exists();

        let history = if record_mode {
            Vec::new()
        } else {
            serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
        };

        Self {
            history_file_path,
            record_mode,
            history: RwLock::new(history),
            access_counter: AtomicU64::new(0),
            fallback,
        }
    }
}

impl ReplacementPolicy for ReplayReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        if self.record_mode {
            return self.fallback.pick_for_eviction(spans);
        }

        let position = self.access_counter.load(Ordering::Relaxed);
        let history = self.history.read().unwrap();
        if position >= history.len() as u64 {
            return self.fallback.pick_for_eviction(spans);
        }

        // pick based on history
        let mut span_pos = vec![usize::MAX; spans.len()];
        for i in 0..spans.len() {
            for k in (position as usize)..history.len() {
                if history[k] == spans[i] {
                    span_pos[i] = k;
                    break;
                }
            }
        }

        let max_pos = span_pos.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        &spans[max_pos]
    }

    fn on_span_access(&self, span_id: &SpanId) {
        if self.record_mode {
            self.history.write().unwrap().push(span_id.clone());
        } else {
            self.access_counter.fetch_add(1, Ordering::Relaxed);
        }

        self.fallback.on_span_access(span_id)
    }

    fn on_span_swap_out(&self, span_id: &SpanId) {
        self.fallback.on_span_swap_out(span_id)
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.fallback.on_span_swap_in(span_id)
    }

    fn on_stop(&self) {
        if self.record_mode {
            fs::write(&self.history_file_path, &serde_json::to_vec(&*self.history.read().unwrap()).unwrap()).unwrap();
        }
    }
}
