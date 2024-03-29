use {
    std::{sync::RwLock, collections::HashMap},
    crate::client::span::SpanId,
    super::FarMemoryBackend,
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
}

impl FarMemoryBackend for InMemoryBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        if prepend {
            let mut spans = self.spans.write().unwrap();
            let mut existing_data = spans.insert(id.clone(), span.to_vec()).unwrap();
            spans.get_mut(&id).unwrap().append(&mut existing_data);
        } else {
            self.spans.write().unwrap().insert(id, span.to_vec());
        }
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        self.spans.write().unwrap().remove(id).unwrap()
    }
}