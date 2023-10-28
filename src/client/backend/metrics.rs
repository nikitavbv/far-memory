use {
    std::time::Instant,
    prometheus::{
        Registry,
        IntCounter,
        Counter,
        register_int_counter_with_registry,
        register_counter_with_registry,
    },
    crate::client::SpanId,
    super::FarMemoryBackend,
};

pub struct InstrumentedBackend {
    inner: Box<dyn FarMemoryBackend>,

    registry: Registry,

    swap_out_bytes: IntCounter,
    swap_out_time_ms: Counter,

    swap_in_bytes: IntCounter,
    swap_in_time_ms: Counter,
}

impl InstrumentedBackend {
    pub fn new(registry: Registry, inner: Box<dyn FarMemoryBackend>) -> Self {
        Self {
            inner,

            registry: registry.clone(),

            swap_out_bytes: register_int_counter_with_registry!(
                "client_backend_swap_out_bytes",
                "total bytes swapped out",
                registry
            ).unwrap(),
            swap_out_time_ms: register_counter_with_registry!(
                "client_backend_swap_out_time",
                "total time spent swapping out",
                registry
            ).unwrap(),

            swap_in_bytes: register_int_counter_with_registry!(
                "client_backend_swap_in_bytes",
                "total bytes swapped in",
                registry
            ).unwrap(),
            swap_in_time_ms: register_counter_with_registry!(
                "client_backend_swap_in_time",
                "total time spent swapping in",
                registry
            ).unwrap(),
        }
    }
}

impl FarMemoryBackend for InstrumentedBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        let started_at = Instant::now();
        self.inner.swap_out(id, span, prepend);
        self.swap_out_time_ms.inc_by((Instant::now() - started_at).as_micros() as f64 / 1000.0);
        self.swap_out_bytes.inc_by(span.len() as u64);
    }

    fn batch_swap_out(&self, swap_out_operations: Vec<super::SwapOutOperation>) {
        let started_at = Instant::now();
        let len = swap_out_operations.iter().map(|op| op.data.len() as u64).sum();
        self.inner.batch_swap_out(swap_out_operations);
        self.swap_out_time_ms.inc_by((Instant::now() - started_at).as_micros() as f64 / 1000.0);
        self.swap_out_bytes.inc_by(len);
    }

    fn batch(&self, swap_out_operations: Vec<super::SwapOutOperation>, swap_in: Option<SpanId>) -> Option<Vec<u8>> {
        let started_at = Instant::now();
        let len = swap_out_operations.iter().map(|op| op.data.len() as u64).sum();
        let res = self.inner.batch(swap_out_operations, swap_in);
        self.swap_out_time_ms.inc_by((Instant::now() - started_at).as_micros() as f64 / 1000.0);
        self.swap_out_bytes.inc_by(len);
        if let Some(res) = res.as_ref() {
            self.swap_in_time_ms.inc_by((Instant::now() - started_at).as_micros() as f64 / 1000.0);
            self.swap_in_bytes.inc_by(res.len() as u64);
        }

        res
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        let started_at = Instant::now();
        let res = self.inner.swap_in(id);
        self.swap_in_time_ms.inc_by((Instant::now() - started_at).as_micros() as f64 / 1000.0);
        self.swap_in_bytes.inc_by(res.len() as u64);
        res
    }

    fn on_stop(&self) {
        self.registry.unregister(Box::new(self.swap_out_bytes.clone())).unwrap();
        self.registry.unregister(Box::new(self.swap_out_time_ms.clone())).unwrap();
        self.registry.unregister(Box::new(self.swap_in_bytes.clone())).unwrap();
        self.registry.unregister(Box::new(self.swap_in_time_ms.clone())).unwrap();

        self.inner.on_stop();
    }
}
