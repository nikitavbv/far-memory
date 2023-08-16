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
        // TODO: call byte buffer here
        Ok(())
    }

    fn write(&mut self, offset: u64, bytes: &[u8]) -> std::io::Result<()> {
        info!("writing blocks");

        let begin_block_index = self.block_for_offset(offset);
        let end_block_index = self.block_for_offset(offset + bytes.len() as u64);

        let mut blocks_data = Vec::new();

        for block in begin_block_index..end_block_index+1 {
            let block_id = self.block_id_for_block_offset(block);
            let mut block_data = self.read_block(&block_id);        
            blocks_data.append(&mut block_data);    
        }

        let blocks_begin_offset = self.offset_for_block(begin_block_index);

        blocks_data[(offset - blocks_begin_offset) as usize..(offset - blocks_begin_offset + bytes.len() as u64) as usize].copy_from_slice(bytes);

        let mut i = 0;
        for block in begin_block_index..end_block_index+1 {
            let block_id = self.block_id_for_block_offset(block);
            let block_data = &blocks_data[(i * self.far_memory_block_size) as usize..((i+1)*self.far_memory_block_size) as usize];
            let block_data = block_data.to_vec();

            self.blocks_cache.put(block_id.clone(), block_data.clone());
            self.write_block(block_id, block_data);

            i += 1;
        }

        info!("done writing blocks");

        Ok(())
    }

    fn block_size(&self) -> u32 {
        1024
    }

    fn blocks(&self) -> u64 {
        1024 * 100
    }
}
