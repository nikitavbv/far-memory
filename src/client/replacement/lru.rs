use {
    std::sync::RwLock,
    tracing::{span, Level},
    lru::LruCache,
    crate::client::{SpanId, ReplacementPolicy},
};

// 108.3 per token (for 25700)
pub struct LeastRecentlyUsedReplacementPolicy {
    cache: RwLock<LruCache<SpanId, ()>>,
}

impl LeastRecentlyUsedReplacementPolicy {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(LruCache::unbounded()),
        }
    }
}

impl ReplacementPolicy for LeastRecentlyUsedReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        let cache = span!(Level::DEBUG, "acquiring cache lock").in_scope(|| self.cache.read().unwrap());
        span!(Level::DEBUG, "filtering spans").in_scope(|| {
            for (span_id, _) in cache.iter() {
                if let Some(index) = spans.iter().position(|v| v == span_id) {
                    return &spans[index];
                }
            }

            &spans[0]
        })
    }

    fn on_new_span(&self, span_id: &SpanId) {
        self.cache.write().unwrap().put(span_id.clone(), ());
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.cache.write().unwrap().promote(span_id);
    }

    fn on_span_swap_out(&self, span_id: &SpanId, partial: bool) {
        if !partial {
            self.cache.write().unwrap().pop_entry(span_id);
        }
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.cache.write().unwrap().put(span_id.clone(), ());
    }
}
