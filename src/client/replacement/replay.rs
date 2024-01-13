use {
    std::sync::{atomic::{AtomicU64, Ordering}, RwLock},
    itertools::Itertools,
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
    fn pick_for_eviction(&self, spans: &[SpanId]) -> Box<dyn Iterator<Item = SpanId>> {
        {
            let span_access_events = self.span_access_events.read().unwrap();
            if !span_access_events.is_empty() {
                return pick_based_on_history(spans, &span_access_events);
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

    fn on_span_swap_out(&self, span_id: &SpanId, partial: bool) {
        self.fallback.on_span_swap_out(span_id, partial)
    }

    fn on_span_swap_in(&self, span_id: &SpanId) {
        self.fallback.on_span_swap_in(span_id)
    }

    fn on_stop(&self) {
        // TODO: flush state
        self.fallback.on_stop()
    }
}

fn pick_based_on_history(spans: &[SpanId], events: &[SpanAccessEvent]) -> Box<dyn Iterator<Item = SpanId>> {
    let mut span_pos = vec![usize::MAX; spans.len()];
    for i in 0..spans.len() {
        for k in 0..events.len() {
            if events[k].span_id == spans[i].id() {
                span_pos[i] = k;
                break;
            }
        }
    }

    let spans = spans.to_vec();
    return Box::new(span_pos.into_iter()
        .enumerate()
        .sorted_by_key(|(_index, value)| *value)
        .map(move |(index, _value)| spans[index].clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let events = vec![
            SpanAccessEvent {
                time_step: 0,
                span_id: 2,
            },
            SpanAccessEvent {
                time_step: 1,
                span_id: 0,
            },
            SpanAccessEvent {
                time_step: 2,
                span_id: 1,
            }
        ];

        let spans = vec![
            SpanId::from_id(0),
            SpanId::from_id(1),
            SpanId::from_id(2),
        ];

        let mut spans_to_swap_out = pick_based_on_history(&spans, &events);
        assert_eq!(Some(SpanId::from_id(2)), spans_to_swap_out.next());
        assert_eq!(Some(SpanId::from_id(0)), spans_to_swap_out.next());
        assert_eq!(Some(SpanId::from_id(1)), spans_to_swap_out.next());
        assert_eq!(None, spans_to_swap_out.next());
    }
}
