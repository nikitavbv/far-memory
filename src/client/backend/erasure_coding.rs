use {
    std::{sync::RwLock, collections::HashMap},
    crate::client::span::SpanId,
    reed_solomon_erasure::galois_8::ReedSolomon,
    super::FarMemoryBackend,
};

pub struct ErasureCodingBackend {
    targets: Vec<Box<dyn FarMemoryBackend>>,
    reed_solomon: ReedSolomon,

    span_length: RwLock<HashMap<SpanId, usize>>,
}

impl ErasureCodingBackend {
    pub fn new(targets: Vec<Box<dyn FarMemoryBackend>>) -> Self {
        Self {
            targets,
            reed_solomon: ReedSolomon::new(3, 2).unwrap(),
            span_length: RwLock::new(HashMap::new()),
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

impl FarMemoryBackend for ErasureCodingBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        let data = if prepend {
            // not optimal at all, but good enough for now
            let mut data = span.to_vec();
            data.append(&mut self.swap_in(&id));
            data
        } else {
            span.to_vec()
        };

        self.span_length.write().unwrap().insert(id.clone(), data.len());

        let shard_len = data.len() / 3;
        let shard_len  = if data.len() % 3 == 0 {
            shard_len
        } else {
            shard_len + 1
        };

        let mut shards = vec![
            data[0..shard_len].to_vec(),
            data[shard_len..2*shard_len].to_vec(),
            {
                let mut third_shard = data[2*shard_len..].to_vec();
                third_shard.resize(shard_len, 0);
                third_shard
            },
            vec![0; shard_len],
            vec![0; shard_len]
        ];

        self.reed_solomon.encode(&mut shards).unwrap();

        // can be optimized to write in parallel. Data can be written in sync, partity - async.
        for i in 0..shards.len() {
            self.targets[i].swap_out(id.clone(), &shards[i], false);
        }
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        let mut shards = Vec::new();

        // can be optimized to read async. First 3 shards are enough to restore data.
        for target in &self.targets {
            shards.push(target.swap_in(id));
        }

        let mut result = shards.remove(0);
        result.append(&mut shards[0]);
        result.append(&mut shards[1]);

        result[0..*self.span_length.read().unwrap().get(id).unwrap()].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use {
        rand::Rng,
        crate::client::InMemoryBackend,
        super::*,
    };

    #[test]
    fn simple_swap_out_swap_in() {
        let data: Vec<u8> = (0..1024).map(|_| rand::thread_rng().gen()).collect();
        let span_id = SpanId::from_id(42);

        let backend = ErasureCodingBackend::new(vec![
            Box::new(InMemoryBackend::new()),
            Box::new(InMemoryBackend::new()),
            Box::new(InMemoryBackend::new()),
            Box::new(InMemoryBackend::new()),
            Box::new(InMemoryBackend::new()),
        ]);

        backend.swap_out(span_id.clone(), &data, false);
        let result = backend.swap_in(&span_id);

        assert_eq!(data, result);
    }
}
