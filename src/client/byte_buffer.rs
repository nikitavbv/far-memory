use {
    std::{collections::HashMap, str::FromStr, num::NonZeroUsize},
    tracing::info,
    tonic::{
        codegen::InterceptedService,
        transport::{Endpoint, Channel},
    },
    super::{
        controller_client::ControllerClient,
        block_map::{LocalBlockMap, RemoteBlockMap},
        block_allocator::BlockAllocator,
        block_writer::WriteRequest,
    },
};

pub struct FarMemoryByteBuffer {
    local_block_map: LocalBlockMap,
    remote_block_map: RemoteBlockMap,
    block_allocator: BlockAllocator,
}

impl FarMemoryByteBuffer {
    pub async fn new(client: ControllerClient, far_memory_block_size: u64) -> Self {
        Self {
            local_block_map: LocalBlockMap::new(far_memory_block_size),
            remote_block_map: RemoteBlockMap::new(),
            block_allocator: BlockAllocator::new(client),
        }
    }

    pub async fn read(&mut self, offset: u64, bytes: &mut [u8]) {
    }

    pub async fn write(&mut self, offset: u64, bytes: &[u8]) {
        let local_blocks = self.local_block_map.local_blocks_for_range(offset, bytes.len() as u64);
        let mut remote_blocks = Vec::new();

        for local_block in local_blocks {
            match self.remote_block_map.remote_block_for_local_block(&local_block) {
                Some(v) => remote_blocks.push(v.clone()),
                None => {
                    let block = self.block_allocator.allocate_block().await;
                    remote_blocks.push(block.clone());
                    self.remote_block_map.put_remote_block_for_local(local_block.clone(), block);
                }
            }
        }

        // TODO: create write requests
        let mut offset = offset;
        let mut bytes_to_write_offset = 0;

        let mut requests: Vec<WriteRequest> = vec![];

        
    }
}