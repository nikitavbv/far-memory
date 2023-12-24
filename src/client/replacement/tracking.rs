use {
    std::sync::atomic::{AtomicU64, Ordering},
    crate::{
        manager::ManagerClient,
        client::SpanId,
    },
    super::ReplacementPolicy,
};

pub struct TrackingReplacementPolicy {
    inner: Box<dyn ReplacementPolicy>,
    manager: ManagerClient,
    step_counter: AtomicU64,
}

impl TrackingReplacementPolicy {
    pub fn new(manager: ManagerClient, inner: Box<dyn ReplacementPolicy>) -> Self {
        Self {
            manager,
            inner,
            step_counter: AtomicU64::new(0),
        }
    }
}

impl ReplacementPolicy for TrackingReplacementPolicy {
    fn pick_for_eviction(&self, spans: &[SpanId]) -> SpanId {
        self.inner.pick_for_eviction(spans)
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.manager.on_span_access(span_id, self.step_counter.fetch_add(1, Ordering::Relaxed));
        self.inner.on_span_access(span_id)
    }

    fn on_span_swap_out(&self, span_id: &SpanId, partial: bool) {
        self.inner.on_span_swap_out(span_id, partial)
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.inner.on_span_swap_in(span_id)
    }

    fn on_stop(&self) {
        // TODO: flush stats
        self.inner.on_stop()
    }
}
