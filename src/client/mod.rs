use {
    std::{io::Error, str::FromStr, collections::HashMap, num::NonZeroUsize, path::Path},
    tracing::info,
    vblk::{mount, BlockDevice},
    tonic::{
        service::Interceptor,
        codegen::InterceptedService,
        transport::{Endpoint, Channel},
        metadata::MetadataValue,
    },
    lru::LruCache,
    crate::{
        utils::AuthInterceptor,
        rpc::{
            memory_storage_service_client::MemoryStorageServiceClient,
            MemoryBlockId,
            AllocateMemoryBlockRequest,
            ReadMemoryBlockRequest,
            WriteMemoryBlockRequest,
        },
    },
    self::byte_buffer::FarMemoryByteBuffer,
};

pub mod byte_buffer;

pub async fn run_block_storage_client(endpoint: String, token: String, far_memory_block_size: u64) {
    info!("starting block storage client");

    let mut device = FarMemoryDevice::new(endpoint, token, far_memory_block_size).await;

    tokio::task::spawn_blocking(move || {
        unsafe {
            mount(&mut device, "/dev/nbd1", |_device| Ok(())).unwrap();
        }
    });
}

struct FarMemoryDevice {
    runtime: tokio::runtime::Runtime,
    byte_buffer: FarMemoryByteBuffer,
}

impl FarMemoryDevice {
    pub async fn new(endpoint: String, token: String, far_memory_block_size: u64) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let mut byte_buffer = runtime.spawn(async move {
            FarMemoryByteBuffer::new(endpoint, token, far_memory_block_size).await
        }).await.unwrap();

        if Path::new("./block_map").exists() {
            byte_buffer.load_block_map(&std::fs::read("./block_map").unwrap());
        }

        Self {
            runtime,
            byte_buffer,
        }
    }
}

impl BlockDevice for FarMemoryDevice {
    fn read(&mut self, offset: u64, bytes: &mut [u8]) -> Result<(), Error> {
        self.runtime.block_on(async {
            self.byte_buffer.read(offset, bytes);
        });
        Ok(())
    }

    fn write(&mut self, offset: u64, bytes: &[u8]) -> std::io::Result<()> {
        self.runtime.block_on(async {
            self.byte_buffer.write(offset, bytes);
        });
        Ok(())
    }

    fn block_size(&self) -> u32 {
        1024
    }

    fn blocks(&self) -> u64 {
        1024 * 100
    }
}
