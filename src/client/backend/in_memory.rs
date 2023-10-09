use {
    std::{sync::RwLock, collections::HashMap},
    crate::client::client::SpanId,
};

pub struct InMemoryBackend {
    spans: RwLock<HashMap<SpanId, Vec<u8>>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            spans: RwLock::new(HashMap::new()),
        }
    }

    pub fn swap_out(&self, id: SpanId, span: &[u8]) {
        self.spans.write().unwrap().insert(id, span.to_vec());
    }
}