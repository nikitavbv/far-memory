use {
    std::sync::Mutex,
    crate::{
        storage::Client,
        client::span::SpanId,
    },
    super::FarMemoryBackend,
};

pub struct NetworkNodeBackend {
    client: Mutex<Client>,
}

impl NetworkNodeBackend {
    pub fn new(endpoint: &str, token: &str) -> Self {
        let mut client = Client::new(endpoint);
        client.auth(token);
        
        Self {
            client: Mutex::new(client),
        }
    }
}

impl FarMemoryBackend for NetworkNodeBackend {
    fn swap_out(&self, id: SpanId, span: &[u8]) {
        self.client.lock().unwrap().swap_out(id.id(), span.to_vec());
    }
    
    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        self.client.lock().unwrap().swap_in(id.id())
    }
}