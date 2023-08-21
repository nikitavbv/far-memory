pub struct BlockMap {
    block_size: u64,
}

impl BlockMap {
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
}

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_blocks_for_range() {
        let map = BlockMap::new(32);

        let blocks_for_range = map.local_blocks_for_range(24, 12);
        assert_eq!(blocks_for_range, vec![LocalBlockId::new(0), LocalBlockId::new(1)]);
    }
}