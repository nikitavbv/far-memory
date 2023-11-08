use {
    std::sync::{Arc, atomic::{AtomicU64, Ordering}},
    crate::{
        manager::ManagerClient,
        client::SpanId,
    },
    super::ReplacementPolicy,
};

pub struct TrackingReplacementPolicy {
    inner: Box<dyn ReplacementPolicy>,
    manager: Arc<ManagerClient>,
    step_counter: AtomicU64,
}

impl TrackingReplacementPolicy {
    pub fn new(manager: Arc<ManagerClient>, inner: Box<dyn ReplacementPolicy>) -> Self {
        Self {
            manager,
            inner,
            step_counter: AtomicU64::new(0),
        }
    }
}

impl ReplacementPolicy for TrackingReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        self.inner.pick_for_eviction(spans)
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.manager.on_span_access(span_id, self.step_counter.fetch_add(1, Ordering::Relaxed));
        self.inner.on_span_access(span_id)
    }

    fn on_span_swap_out(&self, span_id: &SpanId) {
        self.inner.on_span_swap_out(span_id)
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.inner.on_span_swap_in(span_id)
    }

    fn on_stop(&self) {
        // TODO: flush stats
        self.inner.on_stop()
    }
}
