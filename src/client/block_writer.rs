use super::block_map::RemoteBlockId;

pub struct WriteRequest {
    block: RemoteBlockId,
    offset: u32,
    data: u32,
}

impl WriteRequest {
    pub fn new(block: RemoteBlockId, offset: u32, data: u32) -> Self {
        Self {
            block,
            offset,
            data,
        }
    }
}