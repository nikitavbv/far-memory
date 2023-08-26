use super::block_map::RemoteBlockId;

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