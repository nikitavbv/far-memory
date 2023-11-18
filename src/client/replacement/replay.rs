use {
    std::sync::{atomic::{AtomicU64, Ordering}, RwLock},
    crate::{
        manager::{ManagerClient, SpanAccessEvent, ReplacementPolicyType},
        client::SpanId,
    },
    super::ReplacementPolicy,
};

pub struct RemoteReplayReplacementPolicy {
    fallback: Box<dyn ReplacementPolicy>,
    manager: ManagerClient,

    step_counter: AtomicU64,
    span_access_events: RwLock<Vec<SpanAccessEvent>>,
}

impl RemoteReplayReplacementPolicy {
    pub fn new(manager: ManagerClient, fallback: Box<dyn ReplacementPolicy>) -> Self {
        let params = manager.get_replacement_policy_params(ReplacementPolicyType::Replay).span_access_history;

        Self {
            manager,
            fallback,

            step_counter: AtomicU64::new(0),
            span_access_events: RwLock::new(params.unwrap_or(Vec::new())),
        }
    }
}

impl ReplacementPolicy for RemoteReplayReplacementPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a [SpanId]) -> &'a SpanId {
        {
            let span_access_events = self.span_access_events.read().unwrap();
            if !span_access_events.is_empty() {
                // pick based on history
                let mut span_pos = vec![usize::MAX; spans.len()];
                for i in 0..spans.len() {
                    for k in 0..span_access_events.len() {
                        if span_access_events[k].span_id == spans[i].id() {
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

                return &spans[max_pos];
            }
        }
        self.fallback.pick_for_eviction(spans)
    }

    fn on_span_access(&self, span_id: &SpanId) {
        {
            let mut span_access_events = self.span_access_events.write().unwrap();
            if !span_access_events.is_empty() {
                span_access_events.remove(0);
            }
        }

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
