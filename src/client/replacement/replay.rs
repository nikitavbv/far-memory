use {
    std::sync::atomic::{AtomicU64, Ordering},
    crate::{
        manager::{ManagerClient, SpanAccessEvent},
        client::SpanId,
    },
    super::ReplacementPolicy,
};

pub struct RemoteReplayReplacementPolicy {
    fallback: Box<dyn ReplacementPolicy>,
    manager: ManagerClient,

    step_counter: AtomicU64,
    span_access_events: Vec<SpanAccessEvent>,
}

impl RemoteReplayReplacementPolicy {
    pub fn new(manager: ManagerClient, fallback: Box<dyn ReplacementPolicy>) -> Self {
        let params = manager.get_replacement_policy_params().span_access_history;

        Self {
            manager,
            fallback,

            step_counter: AtomicU64::new(0),
            span_access_events: params.unwrap_or(Vec::new()),
        }
    }
}

impl ReplacementPolicy for RemoteReplayReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a [SpanId]) -> &'a SpanId {
        self.fallback.pick_for_eviction(spans)
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.manager.on_span_access(span_id, self.step_counter.fetch_add(1, Ordering::Relaxed));
        self.fallback.on_span_access(span_id)
    }

    fn on_span_swap_out(&self, span_id: &SpanId) {
        self.fallback.on_span_swap_out(span_id)
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.fallback.on_span_swap_in(span_id)
    }

    fn on_stop(&self) {
        // TODO: flush state
        self.fallback.on_stop()
    }
}
