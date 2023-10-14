use {
    std::sync::Mutex,
    crate::{
        storage::Client,
        client::client::SpanId,
        utils::performance::{COUNTER_SWAP_IN, COUNTER_SWAP_OUT, Counter},
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
        let m = Counter::measure();
        self.client.lock().unwrap().swap_out(id.id(), span.to_vec());
        COUNTER_SWAP_OUT.add(m);
    }
    
    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        let m = Counter::measure();
        let res = self.client.lock().unwrap().swap_in(id.id());
        COUNTER_SWAP_IN.add(m);
    
        res
    }
}