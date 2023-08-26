use super::{
    controller_client::ControllerClient,
    block_map::RemoteBlockId,
};

pub struct BlockAllocator {
    client: ControllerClient,
}

impl BlockAllocator {
    pub fn new(client: ControllerClient) -> Self {
        Self {
            client,
        }
    }

    pub async fn allocate_block(&self) -> RemoteBlockId {
        self.client.allocate_block().await
    }
}