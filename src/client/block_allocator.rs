use super::{
    far_memory_client::FarMemoryClient,
    block_map::RemoteBlockId,
};

pub struct BlockAllocator {
    client: FarMemoryClient,
}

impl BlockAllocator {
    pub fn new(client: FarMemoryClient) -> Self {
        Self {
            client,
        }
    }

    pub async fn allocate_block(&self) -> RemoteBlockId {
        self.client.allocate_block().await
    }
}