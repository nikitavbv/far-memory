use {
    std::collections::HashMap,
    tracing::info,
    tonic::{transport::Server, Status, Request, Response},
    tokio::sync::Mutex,
    crate::rpc::{
        memory_storage_service_server::{
            MemoryStorageService,
            MemoryStorageServiceServer,
        },
        MemoryBlockId,
        AllocateMemoryBlockRequest,
        AllocateMemoryBlockResponse,
        WriteMemoryBlockRequest,
        WriteMemoryBlockResponse,
        ReadMemoryBlockRequest,
        ReadMemoryBlockResponse,
        FreeMemoryBlockRequest,
        FreeMemoryBlockResponse,
    }
};

#[derive(Eq, Hash, PartialEq, Clone)]
struct BlockId {
    id: u32,
}

impl From<&MemoryBlockId> for BlockId {
    fn from(value: &MemoryBlockId) -> Self {
        Self {
            id: value.id,
        }
    }
}

struct BlockData {
    data: Vec<u8>,
}

impl BlockData {
    fn new(data: Vec<u8>) -> Self {
        Self {
            data,
        }
    }
}

pub async fn run_memory_storage_server() {
    let addr = "0.0.0.0:9000".parse().unwrap();

    info!("starting memory storage server on {:?}", addr);

    Server::builder()
        .add_service(MemoryStorageServiceServer::new(MemoryStorageServiceHandler::new()))
        .serve(addr)
        .await
        .unwrap();
}

struct MemoryStorageServiceHandler {
    storage: Mutex<HashMap<BlockId, BlockData>>,
    id_counter: Mutex<u32>,
}

impl MemoryStorageServiceHandler {
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(HashMap::new()),
            id_counter: Mutex::new(0),
        }
    }

    async fn next_id(&self) -> BlockId {
        let mut id_counter = self.id_counter.lock().await;
        *id_counter += 1;

        BlockId {
            id: *id_counter,
        }
    }

    fn new_block(&self) -> BlockData {
        BlockData::new(vec![0; 2 * 1024 * 1024])
    }
}

#[tonic::async_trait]
impl MemoryStorageService for MemoryStorageServiceHandler {
    async fn allocate_memory_block(&self, _req: Request<AllocateMemoryBlockRequest>) -> Result<Response<AllocateMemoryBlockResponse>, Status> {
        let id = self.next_id().await;
        let block_data = self.new_block();

        let mut storage = self.storage.lock().await;
        storage.insert(id.clone(), block_data);

        Ok(Response::new(AllocateMemoryBlockResponse { 
            id: Some(MemoryBlockId {
                id: id.id,
            })
        }))
    }

    async fn write_memory_block(&self, req: Request<WriteMemoryBlockRequest>) -> Result<Response<WriteMemoryBlockResponse>, Status> {
        let req = req.into_inner();
        let id = BlockId::from(req.id.as_ref().unwrap());

        let mut storage = self.storage.lock().await;
        storage.get_mut(&id).unwrap().data = req.data;

        Ok(Response::new(WriteMemoryBlockResponse {}))
    }

    async fn read_memory_block(&self, req: Request<ReadMemoryBlockRequest>) -> Result<Response<ReadMemoryBlockResponse>, Status> {
        let req = req.into_inner();
        let id = BlockId::from(req.id.as_ref().unwrap());

        let storage = self.storage.lock().await;
        let data = storage.get(&id).unwrap().data.clone();
        
        Ok(Response::new(ReadMemoryBlockResponse {
            data,
        }))
    }

    async fn free_memory_block(&self, req: Request<FreeMemoryBlockRequest>) -> Result<Response<FreeMemoryBlockResponse>, Status> {
        let req = req.into_inner();
        let id = BlockId::from(req.id.as_ref().unwrap());

        let mut storage = self.storage.lock().await;
        storage.remove(&id);

        Ok(Response::new(FreeMemoryBlockResponse {}))
    }
}