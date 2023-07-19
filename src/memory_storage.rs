use {
    tracing::info,
    tonic::transport::Server,
    crate::rpc::memory_storage_service_server::{
        MemoryStorageService,
        MemoryStorageServiceServer,
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
}