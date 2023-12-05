use {
    aes_gcm::{aead::{KeyInit, Aead, AeadCore}, Aes256Gcm, Key},
    rand::rngs::OsRng,
    crate::client::span::SpanId,
    super::{FarMemoryBackend, SwapOutOperation, SwapOutOperationData},
};

pub struct EncryptionBackend {
    inner: Box<dyn FarMemoryBackend>,
    cipher: Aes256Gcm,
}

impl EncryptionBackend {
    pub fn new(inner: Box<dyn FarMemoryBackend>) -> Self {
        Self {
            inner,
            cipher: Aes256Gcm::new(&Aes256Gcm::generate_key(OsRng)),
        }
    }

    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        assert_eq!(12, nonce.len());

        let encrypted = self.cipher.encrypt(&nonce, data).unwrap();
        {
            let mut encrypted = encrypted;
            let mut result = nonce.to_vec();
            result.append(&mut encrypted);
            result
        }
    }

    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.cipher.decrypt(data[0..12].into(), &data[12..]).unwrap()
    }

    fn encrypt_batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) -> Vec<SwapOutOperation> {
        swap_out_operations.into_iter()
            .map(|v| SwapOutOperation {
                id: v.id,
                data: SwapOutOperationData::Owned(self.encrypt(v.data.as_slice())),
                prepend: v.prepend,
            })
            .collect()
    }
}

impl FarMemoryBackend for EncryptionBackend {
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        if prepend {
            // not optimal, but good enough for now
            let mut span_data = span.to_vec();
            let mut data = self.decrypt(&self.inner.swap_in(&id));
            span_data.append(&mut data);

            let encrypted = self.encrypt(&data);
            self.inner.swap_out(id, &encrypted, false);
        } else {
            self.inner.swap_out(id, & self.encrypt(span), prepend)
        }
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        self.decrypt(&self.inner.swap_in(id))
    }

    fn batch_swap_out(&self, swap_out_operations: Vec<SwapOutOperation>) {
        self.inner.batch_swap_out(self.encrypt_batch_swap_out(swap_out_operations))
    }

    fn batch(&self, swap_out_operations: Vec<SwapOutOperation>, swap_in: Option<&SpanId>) -> Option<Vec<u8>> {
        self.inner.batch(self.encrypt_batch_swap_out(swap_out_operations), swap_in).map(|v| self.decrypt(&v))
    }

    fn on_stop(&self) {
        self.inner.on_stop()
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
    fn simple() {
        let span_id = SpanId::from_id(42);
        let data: Vec<u8> = (0..1024).map(|_| rand::thread_rng().gen()).collect();

        let backend = EncryptionBackend::new(Box::new(InMemoryBackend::new()));

        backend.swap_out(span_id.clone(), &data, false);

        let swapped_in_data = backend.swap_in(&span_id);
        assert_eq!(swapped_in_data, data);
    }
}
