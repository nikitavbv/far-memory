use {
    rand::seq::SliceRandom,
    super::SpanId,
};

pub trait EvictionPolicy: Send + Sync {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId;
}

pub struct RandomEvictionPolicy {
}

impl RandomEvictionPolicy {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl EvictionPolicy for RandomEvictionPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a [SpanId]) -> &'a SpanId {
        spans.choose(&mut rand::thread_rng()).unwrap()
    }
}