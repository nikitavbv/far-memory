use {
    std::io::Error,
    tracing::info,
    vblk::{mount, BlockDevice},
};

pub async fn run_block_storage_client() {
    info!("starting block storage client");

    unsafe {
        mount(&mut FarMemoryDevice, "/dev/nbd0", |_device| Ok(())).unwrap();
    }
}

struct FarMemoryDevice;

impl BlockDevice for FarMemoryDevice {
    fn read(&mut self, offset: u64, bytes: &mut [u8]) -> Result<(), Error> {
        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = match (index as u64 + offset) % 4 {
                0 => 0xDE,
                1 => 0xAD,
                2 => 0xDE,
                _ => 0xEF,
            }
        }

        Ok(())
    }

    fn block_size(&self) -> u32 {
        2 * 1024 * 1024
    }

    fn blocks(&self) -> u64 {
        512
    }
}