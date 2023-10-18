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
    fn swap_out(&self, id: SpanId, span: &[u8], prepend: bool) {
        let path: String = self.path_for_span(&id);
        if prepend {
            let mut data = fs::read(&path).unwrap();
            let mut new_data = span.to_vec();
            new_data.append(&mut data);
            fs::write(path, new_data).unwrap();
        } else {
            fs::write(path, span.to_vec()).unwrap();
        }
    }

    fn swap_in(&self, id: &SpanId) -> Vec<u8> {
        let path: String = self.path_for_span(&id);
        let res = fs::read(&path).unwrap();
        fs::remove_file(path).unwrap();
        res
    }
}