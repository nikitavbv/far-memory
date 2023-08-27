use {
    std::{sync::Arc, str::FromStr, collections::HashMap},
    tokio::sync::Mutex,
    tonic::{codegen::InterceptedService, transport::{Endpoint, Channel}},
    crate::{
        utils::AuthInterceptor,
        rpc::{
            controller_service_client::ControllerServiceClient,
            memory_storage_service_client::MemoryStorageServiceClient,
            ControllerAllocateMemoryBlockRequest,
        },
    },
    super::block_map::RemoteBlockId,
};

#[derive(Clone)]
pub struct FarMemoryClient {
    controller_client: Arc<Mutex<ControllerServiceClient<InterceptedService<Channel, AuthInterceptor>>>>,
    storage_service_nodes: Arc<Mutex<HashMap<u32, Arc<Mutex<MemoryStorageServiceClient<InterceptedService<Channel, AuthInterceptor>>>>>>>,
}

impl FarMemoryClient {
    pub async fn new(endpoint: String, token: String) -> Self {
        Self {
            controller_client: Arc::new(Mutex::new(ControllerServiceClient::with_interceptor(
                Endpoint::from_str(&endpoint).unwrap().connect().await.unwrap(), 
                AuthInterceptor::new(token)
            ))),
            storage_service_nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn allocate_block(&self) -> RemoteBlockId {
        let res = self.controller_client.lock().await.controller_allocate_memory_block(ControllerAllocateMemoryBlockRequest {
        }).await.unwrap().into_inner();

        RemoteBlockId::new(res.node_id.unwrap().id, res.block_id.unwrap().id)
    }

    pub async fn write(&self, block_id: &RemoteBlockId, offset: u32, data: Vec<u8>) {
        let node_id = block_id.node_id();
        let client = self.client_for_storage_node(node_id).await;

        unimplemented!()
    }

    async fn client_for_storage_node(&self, node_id: u32) -> Arc<Mutex<MemoryStorageServiceClient<InterceptedService<Channel, AuthInterceptor>>>> {
        
        unimplemented!("well, let's get it")
    }
}