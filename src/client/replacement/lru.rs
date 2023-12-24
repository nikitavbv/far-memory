use {
    std::{sync::{atomic::{AtomicU64, Ordering}, RwLock}, collections::{HashMap, HashSet}},
    tracing::{span, Level},
    crate::client::{SpanId, ReplacementPolicy},
};

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
    fn pick_for_eviction(&self, spans: &[SpanId]) -> SpanId {
        let history = span!(Level::DEBUG, "acquiring history lock").in_scope(|| self.history.read().unwrap());
        span!(Level::DEBUG, "filtering spans").in_scope(|| spans.iter()
            .map(|v| (v, history.get(v)))
            .filter(|v| v.1.is_some())
            .map(|v| (v.0, v.1.unwrap()))
            .reduce(|a, b| if a.1 < b.1 { a } else { b }).map(|a| a.0).unwrap().clone())
    }

    fn on_new_span(&self, span_id: &SpanId) {
        self.on_span_access(span_id);
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.history.write().unwrap().insert(span_id.clone(), self.counter.fetch_add(1, Ordering::Relaxed));
    }

    fn on_span_swap_out(&self, span_id: &SpanId, partial: bool) {
        if !partial {
            self.history.write().unwrap().remove(&span_id);
        }
    }

    fn on_span_swap_in(&self, _span_id: &SpanId) {
        // do nothing
    }
}
