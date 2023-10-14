use {
    std::{sync::Mutex, fmt::Debug},
    crate::{
        storage::Client,
        client::client::SpanId,
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

impl Debug for NetworkNodeBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "NetworkNodeBackend".fmt(f)
    }
}

impl FarMemoryBackend for NetworkNodeBackend {
    #[tracing::instrument]
    fn swap_out(&self, id: SpanId, span: &[u8]) {
        self.client.lock().unwrap().swap_out(id.id(), span.to_vec());
    }
    
    #[tracing::instrument]
    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        self.client.lock().unwrap().swap_in(id.id())
    }
}