use {
    std::{collections::HashMap, str::FromStr, num::NonZeroUsize},
    tracing::info,
    tonic::{
        codegen::InterceptedService,
        transport::{Endpoint, Channel},
    },
    lru::LruCache,
    crate::{
        utils::AuthInterceptor,
        rpc::{
            memory_storage_service_client::MemoryStorageServiceClient,
            MemoryBlockId,
            AllocateMemoryBlockRequest,
            ReadMemoryBlockRequest,
            WriteMemoryBlockRequest,
        }
    }
};

pub struct FarMemoryByteBuffer {
    client: MemoryStorageServiceClient<InterceptedService<Channel, AuthInterceptor>>,

    far_memory_block_size: u64,
    blocks_initialized: HashMap<u64, MemoryBlockId>,
    blocks_cache: LruCache<MemoryBlockId, Vec<u8>>,
}

impl FarMemoryByteBuffer {
    pub async fn new(endpoint: String, token: String, far_memory_block_size: u64) -> Self {
        let client = MemoryStorageServiceClient::with_interceptor(
            Endpoint::from_str(&endpoint).unwrap().connect().await.unwrap(), 
            AuthInterceptor::new(token)
        );

        Self {
            client,
            far_memory_block_size,
            blocks_initialized: HashMap::new(),
            blocks_cache: LruCache::new(NonZeroUsize::new(10).unwrap()),
        }
    }

    pub fn load_block_map(&mut self, block_map: &[u8]) {
        self.blocks_initialized = bincode::deserialize(block_map).unwrap();
    }

    fn block_for_offset(&self, offset: u64) -> u64 {
        offset / self.far_memory_block_size
    }

    fn offset_for_block(&self, block_offset: u64) -> u64 {
        block_offset * self.far_memory_block_size
    }

    async fn block_id_for_block_offset(&mut self, block_offset: u64) -> MemoryBlockId {
        if let Some(id) = self.blocks_initialized.get(&block_offset) {
            return id.clone();
        }

        let res = self.client.allocate_memory_block(AllocateMemoryBlockRequest {}).await.unwrap().into_inner();

        info!("initialized block: {:?}", res);

        let id = res.id.unwrap();
        self.blocks_initialized.insert(block_offset, id.clone());

        let block_map = bincode::serialize(&self.blocks_initialized).unwrap();
        std::fs::write("./block_map", &block_map).unwrap();

        id
    }

    async fn read_block(&mut self, block_id: &MemoryBlockId) -> Vec<u8> {
        self.client.read_memory_block(ReadMemoryBlockRequest {
            id: Some(block_id.clone()),
        }).await.unwrap().into_inner().data
    }

    async fn write_block(&mut self, id: MemoryBlockId, data: Vec<u8>) {
        self.client.write_memory_block(WriteMemoryBlockRequest {
            id: Some(id),
            data,
        }).await.unwrap();
    }

    async fn read(&self, offset: u64, bytes: &mut [u8]) {
        let begin_block_index = self.block_for_offset(offset);
        let end_block_index = self.block_for_offset(offset + bytes.len() as u64);

        let mut blocks_data = Vec::new();

        for block in begin_block_index..end_block_index+1 {
            let block_id = self.block_id_for_block_offset(block);

            let block_from_cache = self.blocks_cache.get(&block_id);
            let mut block_data = match block_from_cache {
                Some(v) => v.clone(),
                None => {
                    let data = self.read_block(&block_id);
                    self.blocks_cache.put(block_id.clone(), data.clone());
                    data
                }
            };
            
            blocks_data.append(&mut block_data);    
        }

        let blocks_begin_offset = self.offset_for_block(begin_block_index);

        bytes.copy_from_slice(&blocks_data[(offset - blocks_begin_offset) as usize..(offset - blocks_begin_offset + bytes.len() as u64) as usize]);
    }

    async fn write(&self, offset: u64, bytes: &[u8]) {
        // TODO: implement this
    }
}