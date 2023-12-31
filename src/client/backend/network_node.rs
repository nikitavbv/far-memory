use {
    std::sync::Mutex,
    tracing::debug_span,
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
        debug_span!("waiting for network client lock for swap out").in_scope(|| self.client.lock().unwrap()).swap_out(id.id(), span.to_vec(), prepend);
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        debug_span!("waiting for network client lock for swap in").in_scope(|| self.client.lock().unwrap()).swap_in(id.id())
    }

    fn batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) {
        self.batch(swap_out_operations, None);
    }

    fn batch(&self, swap_out_operations: Vec<SwapOutOperation>, swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        debug_span!("waiting for network client lock for batch operation").in_scope(|| self.client.lock().unwrap()).batch(swap_out_operations.into_iter().map(|v| BatchSwapOutOperation {
            span_id: v.id.id(),
            data: match v.data {
                SwapOutOperationData::Owned(v) => LocalSpanData::Owned(v),
                SwapOutOperationData::ReadFrom { ptr, size } => LocalSpanData::ReadFrom { ptr, size },
            },
            prepend: v.prepend,
        }).collect(), swap_in.map(|v| v.id()))
    }
}
