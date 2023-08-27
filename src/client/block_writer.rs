use {
    super::block_map::RemoteBlockId,
    super::{
        far_memory_client::FarMemoryClient,
    }
};

#[derive(Debug)]
pub struct WriteRequest {
    block: RemoteBlockId,
    offset: u32,
    data: Vec<u8>,
}

impl WriteRequest {
    pub fn new(block: RemoteBlockId, offset: u32, data: Vec<u8>) -> Self {
        Self {
            block,
            offset,
            data,
        }
    }
}

pub struct BlockWriter {
    client: FarMemoryClient,
}

impl BlockWriter {
    pub fn new(client: FarMemoryClient) -> Self {
        Self {
            client,
        }
    }

    pub async fn perform_write(&self, req: WriteRequest) {
        self.client.write(&req.block, req.offset, req.data).await;
    }
}