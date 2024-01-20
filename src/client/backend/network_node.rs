use {
    std::{sync::Mutex, ops::DerefMut},
    tracing::debug_span,
    tokio::runtime::Runtime,
    crate::{
        storage::{Client, SwapOutRequest, SpanData, BatchSwapOutOperation, LocalSpanData},
        client::span::SpanId,
    },
    super::{FarMemoryBackend, SwapOutOperation, SwapOutOperationData},
};

pub struct NetworkNodeBackend {
    client: Mutex<(Runtime, Client)>,
}

impl NetworkNodeBackend {
    pub fn new(endpoint: &str, token: &str, run_id: String) -> Self {
        let runtime = Runtime::new().unwrap();

        let client = runtime.block_on(async {
            let mut client = Client::new(endpoint).await;
            client.auth(token).await;
            client.set_run_id(run_id).await;

            client
        });

        Self {
            client: Mutex::new((runtime, client)),
        }
    }
}

impl FarMemoryBackend for NetworkNodeBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        let lock_span = debug_span!("waiting for network client lock for swap out");
        let lock_span_guard = lock_span.enter();
        let mut lock = self.client.lock().unwrap();
        let (runtime, client) = lock.deref_mut();
        drop(lock_span_guard);
        runtime.block_on(async { client.swap_out(id.id(), span.to_vec(), prepend).await });
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        let lock_span = debug_span!("waiting for network client lock for swap in");
        let lock_span_guard = lock_span.enter();
        let mut lock = self.client.lock().unwrap();
        let (runtime, client) = lock.deref_mut();
        drop(lock_span_guard);
        runtime.block_on(async { client.swap_in(id.id()).await })
    }

    fn batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) {
        self.batch(swap_out_operations, None);
    }

    fn batch(&self, swap_out_operations: Vec<SwapOutOperation>, swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        let lock_span = debug_span!("waiting for network client lock for batch operation");
        let lock_span_guard = lock_span.enter();
        let mut lock = self.client.lock().unwrap();
        let (runtime, client) = lock.deref_mut();
        drop(lock_span_guard);

        runtime.block_on(async {
            client.batch(swap_out_operations.into_iter().map(|v| BatchSwapOutOperation {
                span_id: v.id.id(),
                data: match v.data {
                    SwapOutOperationData::Owned(v) => LocalSpanData::Owned(v),
                    SwapOutOperationData::ReadFrom { ptr, size } => LocalSpanData::ReadFrom { ptr, size },
                },
                prepend: v.prepend,
            }).collect(), swap_in.map(|v| v.id())).await
        })
    }
}
