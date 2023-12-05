use {
    std::io::{Write, Read},
    lz4::{EncoderBuilder, Decoder},
    crate::client::span::SpanId,
    super::{FarMemoryBackend, SwapOutOperation, SwapOutOperationData},
};

pub struct CompressionBackend {
    inner: Box<dyn FarMemoryBackend>,
}

impl CompressionBackend {
    pub fn new(inner: Box<dyn FarMemoryBackend>) -> Self {
        Self {
            inner,
        }
    }

    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();

        let mut encoder = EncoderBuilder::new()
            .level(4)
            .build(&mut output)
            .unwrap();

        encoder.write(&data).unwrap();

        output
    }

    fn decompress(&self, mut data: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        let mut decoder = Decoder::new(&mut data).unwrap();
        decoder.read_to_end(&mut output).unwrap();
        output
    }

    fn compress_batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) -> Vec<SwapOutOperation> {
        swap_out_operations.into_iter()
            .map(|v| SwapOutOperation {
                id: v.id,
                data: SwapOutOperationData::Owned(self.compress(v.data.as_slice())),
                prepend: v.prepend,
            })
            .collect()
    }
}

impl FarMemoryBackend for CompressionBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        if prepend {
            // not optimal, but good enough for now
            let mut span_data = span.to_vec();
            let mut data = self.decompress(&self.inner.swap_in(&id));
            span_data.append(&mut data);

            let compressed = self.compress(&data);
            self.inner.swap_out(id, &compressed, false)
        } else {
            self.inner.swap_out(id, &self.compress(span), prepend)
        }
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        self.decompress(&self.inner.swap_in(id))
    }

    fn batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) {
        self.inner.batch_swap_out(self.compress_batch_swap_out(swap_out_operations))
    }

    fn batch(&self, swap_out_operations: Vec<SwapOutOperation>, swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        self.inner.batch(self.compress_batch_swap_out(swap_out_operations), swap_in).map(|v| self.decompress(&v))
    }

    fn on_stop(&self) {
        self.inner.on_stop()
    }
}

// TODO: add tests
