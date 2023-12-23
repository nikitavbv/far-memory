use {
    std::{sync::{atomic::{AtomicU64, Ordering}, RwLock}, collections::{HashMap, HashSet}},
    tracing::{span, Level},
    crate::client::{SpanId, ReplacementPolicy},
};

// 108.3 per token (for 25700)
pub struct LeastRecentlyUsedReplacementPolicy {
    counter: AtomicU64,
    history: RwLock<HashMap<SpanId, u64>>,
    remote_spans: RwLock<HashSet<SpanId>>,
}

impl LeastRecentlyUsedReplacementPolicy {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            history: RwLock::new(HashMap::new()),
            remote_spans: RwLock::new(HashSet::new()),
        }
    }
}

impl ReplacementPolicy for LeastRecentlyUsedReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        let history = span!(Level::DEBUG, "acquiring history lock").in_scope(|| self.history.read().unwrap());
        let remote_spans = span!(Level::DEBUG, "acquiring remote spans lock").in_scope(|| self.remote_spans.read().unwrap());
        span!(Level::DEBUG, "filtering spans").in_scope(|| spans.iter()
            .filter(|v| !remote_spans.contains(v))
            .map(|v| (v, history.get(v).unwrap_or(&0)))
            .reduce(|a, b| if a.1 < b.1 { a } else { b }).map(|a| a.0).unwrap())
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.history.write().unwrap().insert(span_id.clone(), self.counter.fetch_add(1, Ordering::Relaxed));;
    }

    fn on_span_swap_out(&self, span_id: &SpanId, partial: bool) {
        if !partial {
            self.remote_spans.write().unwrap().insert(span_id.clone());
        }
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.remote_spans.write().unwrap().remove(span_id);
    }
}
