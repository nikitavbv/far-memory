use {
    tracing::info,
    tonic::{transport::Server, Status, Request, Response},
    crate::rpc::{
        memory_storage_service_server::{
            MemoryStorageService,
            MemoryStorageServiceServer,
        },
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
}

impl MemoryStorageServiceHandler {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[tonic::async_trait]
impl MemoryStorageService for MemoryStorageServiceHandler {
    async fn allocate_memory_block(&self, request: Request<AllocateMemoryBlockRequest>) -> Result<Response<AllocateMemoryBlockResponse>, Status> {
        unimplemented!()
    }

    async fn write_memory_block(&self, request: Request<WriteMemoryBlockRequest>) -> Result<Response<WriteMemoryBlockResponse>, Status> {
        unimplemented!()
    }

    async fn read_memory_block(&self, request: Request<ReadMemoryBlockRequest>) -> Result<Response<ReadMemoryBlockResponse>, Status> {
        unimplemented!()
    }

    async fn free_memory_block(&self, request: Request<FreeMemoryBlockRequest>) -> Result<Response<FreeMemoryBlockResponse>, Status> {
        unimplemented!()
    }
}