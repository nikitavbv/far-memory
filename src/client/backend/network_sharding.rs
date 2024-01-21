use {
    std::sync::Mutex,
    tracing::debug_span,
    tokio::runtime::Runtime,
    crate::{
        storage::{Client, BatchSwapOutOperation, LocalSpanData},
        client::span::SpanId,
    },
    super::{FarMemoryBackend, SwapOutOperation, SwapOutOperationData},
};

pub struct NetworkShardingBackend {
    lock: Mutex<()>,
    runtime: Runtime,

    // TODO: refactor this into vec, lol.
    client0: Mutex<Client>,
    client1: Mutex<Client>,
    client2: Mutex<Client>,
    client3: Mutex<Client>,
}

impl NetworkShardingBackend {
    pub fn new(token: &str, run_id: &str, endpoints: Vec<String>) -> Self {
        let runtime = Runtime::new().unwrap();

        let mut clients = runtime.block_on(async move {
            let tasks = endpoints.iter().map(|endpoint| async {
                let mut client = Client::new(endpoint).await;
                client.auth(token).await;
                client.set_run_id(run_id.to_owned()).await;

                client
            }).collect::<Vec<_>>();

            futures::future::join_all(tasks).await
        });

        Self {
            lock: Mutex::new(()),
            runtime,

            client0: Mutex::new(clients.remove(0)),
            client1: Mutex::new(clients.remove(0)),
            client2: Mutex::new(clients.remove(0)),
            client3: Mutex::new(clients.remove(0)),
        }
    }
}

impl FarMemoryBackend for NetworkShardingBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        unimplemented!()
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        let lock_span = debug_span!("waiting for network client lock for swap in");
        let lock_span_guard = lock_span.enter();
        let _lock = self.lock.lock().unwrap();
        drop(lock_span_guard);

        self.runtime.block_on(async {
            let mut client = match id.id() % 4 {
                0 => &self.client0,
                1 => &self.client1,
                2 => &self.client2,
                3 => &self.client3,
                _ => unreachable!(),
            }.lock().unwrap();
            client.swap_in(id.id()).await
        })
    }

    fn batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) {
        self.batch(swap_out_operations, None);
    }

    fn batch(&self, swap_out_operations: Vec<SwapOutOperation>, swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        let lock_span = debug_span!("waiting for sharding lock for batch operation");
        let lock_span_guard = lock_span.enter();
        let _lock = self.lock.lock().unwrap();
        drop(lock_span_guard);

        let mut client0 = self.client0.lock().unwrap();
        let mut client1 = self.client1.lock().unwrap();
        let mut client2 = self.client2.lock().unwrap();
        let mut client3 = self.client3.lock().unwrap();

        self.runtime.block_on(async {
            let swap_out_operations: Vec<_> = swap_out_operations.into_iter().map(|v| BatchSwapOutOperation {
                span_id: v.id.id(),
                data: match v.data {
                    SwapOutOperationData::Owned(v) => LocalSpanData::Owned(v),
                    SwapOutOperationData::ReadFrom { ptr, size } => LocalSpanData::ReadFrom { ptr, size },
                },
                prepend: v.prepend,
            }).collect();

            let mut task0 = Vec::new();
            let swap_in0 = if let Some(span_id) = swap_in {
                if span_id.id() % 4 == 0 {
                    Some(span_id.id())
                } else {
                    None
                }
            } else {
                None
            };

            let mut task1 = Vec::new();
            let swap_in1 = if let Some(span_id) = swap_in {
                if span_id.id() % 4 == 1 {
                    Some(span_id.id())
                } else {
                    None
                }
            } else {
                None
            };

            let mut task2 = Vec::new();
            let swap_in2 = if let Some(span_id) = swap_in {
                if span_id.id() % 4 == 2 {
                    Some(span_id.id())
                } else {
                    None
                }
            } else {
                None
            };

            let mut task3 = Vec::new();
            let swap_in3 = if let Some(span_id) = swap_in {
                if span_id.id() % 4 == 3 {
                    Some(span_id.id())
                } else {
                    None
                }
            } else {
                None
            };

            for operation in swap_out_operations {
                match operation.span_id % 4 {
                    0 => &mut task0,
                    1 => &mut task1,
                    2 => &mut task2,
                    3 => &mut task3,
                    _ => unreachable!(),
                }.push(operation);
            }

            let task0 = client0.batch(task0, swap_in0);
            let task1 = client1.batch(task1, swap_in1);
            let task2 = client2.batch(task2, swap_in2);
            let task3 = client3.batch(task3, swap_in3);

            let (result0, result1, result2, result3) = futures::future::join4(
                task0,
                task1,
                task2,
                task3,
            ).await;

            if result0.is_some() {
                return result0;
            }
            if result1.is_some() {
                return result1;
            }
            if result2.is_some() {
                return result2;
            }
            if result3.is_some() {
                return result3;
            }

            None
        })
    }
}
