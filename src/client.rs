use {
    std::io::Error,
    tracing::info,
    vblk::{mount, BlockDevice},
};

pub async fn run_block_storage_client() {
    info!("starting block storage client");

    let mut device = FarMemoryDevice::new();

    tokio::task::spawn_blocking(move || {
        unsafe {
            mount(&mut device, "/dev/nbd1", |_device| Ok(())).unwrap();
        }
    });
}

struct FarMemoryDevice {
    data: Vec<u8>,
}

impl FarMemoryDevice {
    pub fn new() -> Self {
        Self {
            data: vec![0; 1024 * 1024 * 1024],
        }
    }
}

impl BlockDevice for FarMemoryDevice {
    fn read(&mut self, offset: u64, bytes: &mut [u8]) -> Result<(), Error> {
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn write(&mut self, offset: u64, bytes: &[u8]) -> std::io::Result<()> {
        self.data[offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    fn block_size(&self) -> u32 {
        1024
    }

    fn blocks(&self) -> u64 {
        1024 * 1024
    }
}
