use {
    std::sync::RwLock,
    tracing::{span, Level},
    crate::client::{SpanId, ReplacementPolicy},
};

// 108.3 per token (for 25700)
pub struct LeastRecentlyUsedReplacementPolicy {
    history: RwLock<Vec<SpanId>>,
}

impl LeastRecentlyUsedReplacementPolicy {
    pub fn new() -> Self {
        Self {
            history: RwLock::new(Vec::new()),
        }
    }
}

impl ReplacementPolicy for LeastRecentlyUsedReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        let history = span!(Level::DEBUG, "acquiring history lock").in_scope(|| self.history.read().unwrap());
        span!(Level::DEBUG, "filtering spans").in_scope(|| {
            for i in 0..history.len() {
                if let Some(index) = spans.iter().position(|v| v == &history[i]) {
                    return &spans[index];
                }
            }
            &spans[0]
        })
    }

    fn on_new_span(&self, span_id: &SpanId) {
        self.on_span_access(span_id);
    }

    fn on_span_access(&self, span_id: &SpanId) {
        let mut history = self.history.write().unwrap();
        if let Some(index) = history.iter().position(|v| v == span_id) {
            history.remove(index);
        }
        history.push(span_id.clone());
    }

    fn on_span_swap_out(&self, span_id: &SpanId, partial: bool) {
        if !partial {
            let mut history = self.history.write().unwrap();
            if let Some(index) = history.iter().position(|v| v == span_id) {
                history.remove(index);
            }
        }
    }

    fn on_span_swap_in(&self, _span_id: &SpanId) {
        // do nothing
    }
}
