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
    },
};

pub struct FarMemoryByteBuffer {
    local_block_map: LocalBlockMap,
    remote_block_map: RemoteBlockMap,
}

impl FarMemoryByteBuffer {
    pub async fn new(client: ControllerClient, far_memory_block_size: u64) -> Self {
        Self {
            local_block_map: LocalBlockMap::new(far_memory_block_size),
            remote_block_map: RemoteBlockMap::new(),
        }
    }

    pub async fn read(&mut self, offset: u64, bytes: &mut [u8]) {
    }

    pub async fn write(&mut self, offset: u64, bytes: &[u8]) {
        let local_blocks = self.local_block_map.local_blocks_for_range(offset, bytes.len() as u64);
        let mut remote_blocks = Vec::new();

        for local_block in local_blocks {
            match self.remote_block_map.remote_block_for_local_block(&local_block) {
                Some(v) => remote_blocks.push(v),
                None => {
                    // TODO: request block and save it to remote block map
                    unimplemented!("block allocation is not implemented yet");
                }
            }
        }
    }
}