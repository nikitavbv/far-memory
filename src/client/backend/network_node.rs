use {
    std::sync::Mutex,
    crate::{
        storage::{Client, SwapOutRequest, SpanData, BatchSwapOutOperation, LocalSpanData},
        client::span::SpanId,
    },
    super::{FarMemoryBackend, SwapOutOperation, SwapOutOperationData},
};

pub struct NetworkNodeBackend {
    client: Mutex<Client>,
}

impl NetworkNodeBackend {
    pub fn new(endpoint: &str, token: &str, run_id: String) -> Self {
        let mut client = Client::new(endpoint);
        client.auth(token);
        client.set_run_id(run_id);

        Self {
            client: Mutex::new(client),
        }
    }
}

impl FarMemoryBackend for NetworkNodeBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        self.client.lock().unwrap().swap_out(id.id(), span.to_vec(), prepend);
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        self.client.lock().unwrap().swap_in(id.id())
    }

    fn batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) {
        self.batch(swap_out_operations, None);
    }

    fn batch(&self, swap_out_operations: Vec<SwapOutOperation>, swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        self.client.lock().unwrap().batch(swap_out_operations.into_iter().map(|v| BatchSwapOutOperation {
            span_id: v.id.id(),
            data: LocalSpanData::Owned(v.data.as_slice().to_vec()),
            prepend: v.prepend,
        }).collect(), swap_in.map(|v| v.id()))
    }
}
