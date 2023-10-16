use {
    std::fs,
    crate::client::span::SpanId,
    super::FarMemoryBackend,
};

const PATH: &str = "./data/spans";

pub struct LocalDiskBackend {
}

impl LocalDiskBackend {
    pub fn new() -> Self {
        fs::create_dir_all(PATH).unwrap();

        Self {
        }
    }

    fn path_for_span(&self, id: &SpanId) -> String {
        format!("{}/{}", PATH, id.id())
    }
}

impl FarMemoryBackend for LocalDiskBackend {
    fn swap_out(&self, id: SpanId, span: &[u8]) {
        fs::write(self.path_for_span(&id), span).unwrap();
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        fs::read(self.path_for_span(id)).unwrap()
    }
}