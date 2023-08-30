use {
    tracing::info,
    tonic::{
        codegen::InterceptedService,
        transport::{Endpoint, Channel},
    },
    super::{
        far_memory_client::FarMemoryClient,
        block_map::{LocalBlockMap, RemoteBlockMap},
        block_allocator::BlockAllocator,
        block_writer::{WriteRequest, BlockWriter},
    },
};

pub struct FarMemoryByteBuffer {
    local_block_map: LocalBlockMap,
    remote_block_map: RemoteBlockMap,
    block_allocator: BlockAllocator,
    block_writer: BlockWriter,
}

impl FarMemoryByteBuffer {
    pub async fn new(client: FarMemoryClient, far_memory_block_size: u64) -> Self {
        Self {
            local_block_map: LocalBlockMap::new(far_memory_block_size),
            remote_block_map: RemoteBlockMap::new(),
            block_allocator: BlockAllocator::new(client.clone()),
            block_writer: BlockWriter::new(client),
        }
    }

    pub async fn read(&mut self, offset: u64, bytes: &mut [u8]) {
    }

    pub async fn write(&mut self, offset: u64, bytes: &[u8]) {
        let local_blocks = self.local_block_map.local_blocks_for_range(offset, bytes.len() as u64);
        let mut remote_blocks = Vec::new();

        for (local_block, block_slice) in local_blocks {
            match self.remote_block_map.remote_block_for_local_block(&local_block) {
                Some(v) => remote_blocks.push((v.clone(), block_slice)),
                None => {
                    let block = self.block_allocator.allocate_block().await;
                    remote_blocks.push((block.clone(), block_slice));
                    self.remote_block_map.put_remote_block_for_local(local_block.clone(), block);
                }
            }
        }

        let mut write_requests = vec![];
        let mut bytes_offset = 0;

        for (remote_block, block_slice) in remote_blocks {
            write_requests.push(WriteRequest::new(remote_block, block_slice.offset() as u32, bytes[bytes_offset..(block_slice.len() as usize)].to_vec()));
            bytes_offset += block_slice.len() as usize;
        }

        for request in write_requests {
            self.block_writer.perform_write(request).await;
        }
    }
}
