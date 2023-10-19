use {
    std::{sync::{atomic::{AtomicU64, Ordering}, RwLock}, collections::HashMap},
    rand::seq::SliceRandom,
    super::SpanId,
};

pub trait EvictionPolicy: Send + Sync {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId;

    fn on_span_access(&self, span_id: &SpanId) {}
}

// 6.01 per token
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

// 108.3 per token
pub struct LeastRecentlyUsedEvictionPolicy {
    counter: AtomicU64,
    history: RwLock<HashMap<SpanId, u64>>,
}

impl LeastRecentlyUsedEvictionPolicy {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            history: RwLock::new(HashMap::new()),
        }
    }
}

impl EvictionPolicy for LeastRecentlyUsedEvictionPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        let history = self.history.read().unwrap();
        spans.iter().map(|v| (v, history.get(v).unwrap_or(&0))).reduce(|a, b| if a.1 < b.1 { a } else { b }).map(|a| a.0).unwrap()
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.history.write().unwrap().insert(span_id.clone(), self.counter.fetch_add(1, Ordering::Relaxed));;
    }
}

// 5.42 per token
pub struct MostRecentlyUsedEvictionPolicy {
    counter: AtomicU64,
    history: RwLock<HashMap<SpanId, u64>>,
}

impl MostRecentlyUsedEvictionPolicy {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            history: RwLock::new(HashMap::new()),
        }
    }
}

impl EvictionPolicy for MostRecentlyUsedEvictionPolicy {
    fn pick_for_eviction<'a>(&self, spans: &'a[SpanId]) -> &'a SpanId {
        let history = self.history.read().unwrap();
        spans.iter().map(|v| (v, history.get(v).unwrap_or(&0))).reduce(|a, b| if a.1 > b.1 { a } else { b }).map(|a| a.0).unwrap()
    }

    fn on_span_access(&self, span_id: &SpanId) {
        self.history.write().unwrap().insert(span_id.clone(), self.counter.fetch_add(1, Ordering::Relaxed));;
    }
}