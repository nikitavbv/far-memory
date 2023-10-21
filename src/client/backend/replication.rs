use {
    crate::client::span::SpanId,
    tracing::{span, Level},
    super::FarMemoryBackend,
};

pub struct ReplicationBackend {
    targets: Vec<Box<dyn FarMemoryBackend>>,
}

impl ReplicationBackend {
    pub fn new(targets: Vec<Box<dyn FarMemoryBackend>>) -> Self {
        Self {
            targets,
        }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn with_target(mut self, target: Box<dyn FarMemoryBackend>) -> Self {
        self.targets.push(target);
        self
    }
}

impl FarMemoryBackend for ReplicationBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        span!(Level::DEBUG, "replication write").in_scope(|| {
            // should be parallel, but keeping everything in one thread for now for simplicity
            for target in &self.targets {
                target.swap_out(id.clone(), span, prepend);
            }
        });
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        // can be optimized to run in parallel or to read from one and delete from another
        span!(Level::DEBUG, "replication read").in_scope(|| {
            let res = self.targets[0].swap_in(id);
            for target in &self.targets[1..] {
                target.swap_in(id); // dropped intentionally (see note above)
            }
            res
        })
    }
}
