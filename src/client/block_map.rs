use std::collections::HashMap;

pub struct LocalBlockMap {
    block_size: u64,
}

impl LocalBlockMap {
    pub fn new(block_size: u64) -> Self {
        Self {
            block_size,
        }
    }

    pub fn local_blocks_for_range(&self, offset: u64, len: u64) -> Vec<LocalBlockId> {
        let mut result = Vec::new();
        let start_local_id = offset / self.block_size;  
        let mut i = start_local_id * self.block_size;
        let mut local_id = start_local_id;
        
        while i < offset + len {
            result.push(LocalBlockId::new(local_id));
            i += self.block_size;
            local_id += 1;
        }

        result
    }

    pub fn offset_for_block(&self, block: &LocalBlockId) -> u64 {
        block.id * self.block_size
    }
}

pub struct RemoteBlockMap {
    map: HashMap<LocalBlockId, RemoteBlockId>,
}

impl RemoteBlockMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn remote_block_for_local_block(&self, local_block: &LocalBlockId) -> Option<&RemoteBlockId> {
        self.map.get(local_block)
    }

    pub fn put_remote_block_for_local(&mut self, local_block: LocalBlockId, remote_block: RemoteBlockId) {
        self.map.insert(local_block, remote_block);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LocalBlockId {
    id: u64,
}

impl LocalBlockId {
    pub fn new(id: u64) -> Self {
        Self {
            id,
        }
    }
}

#[derive(Clone)]
pub struct RemoteBlockId {
    node_id: u32,
    block_id: u32,
}

impl RemoteBlockId {
    pub fn new(node_id: u32, block_id: u32) -> Self {
        Self {
            node_id,
            block_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_blocks_for_range() {
        let map = LocalBlockMap::new(32);

        let blocks_for_range = map.local_blocks_for_range(24, 12);
        assert_eq!(blocks_for_range, vec![LocalBlockId::new(0), LocalBlockId::new(1)]);
    }
}