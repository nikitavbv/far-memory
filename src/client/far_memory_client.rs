use {
    std::{sync::Arc, str::FromStr, collections::HashMap},
    tokio::sync::Mutex,
    tonic::{codegen::InterceptedService, transport::{Endpoint, Channel}},
    crate::{
        utils::AuthInterceptor,
        rpc::{
            self,
            controller_service_client::ControllerServiceClient,
            memory_storage_service_client::MemoryStorageServiceClient,
            StorageNodeId,
            ControllerAllocateMemoryBlockRequest,
            GetMemoryStorageNodeByIdRequest,
            WriteMemoryBlockRequest,
        },
    },
    super::block_map::RemoteBlockId,
};

#[derive(Clone)]
pub struct FarMemoryClient {
    token: String,
    controller_client: Arc<Mutex<ControllerServiceClient<InterceptedService<Channel, AuthInterceptor>>>>,
    storage_service_nodes: Arc<Mutex<HashMap<u32, Arc<Mutex<MemoryStorageServiceClient<InterceptedService<Channel, AuthInterceptor>>>>>>>,
}

impl FarMemoryClient {
    pub async fn new(endpoint: String, token: String) -> Self {
        Self {
            token: token.clone(),
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

        client.lock().await.write_memory_block(WriteMemoryBlockRequest {
            id: Some(rpc::MemoryBlockId { id: block_id.block_id() }),
            data,
            offset,
        }).await.unwrap();

        unimplemented!()
    }

    async fn client_for_storage_node(&self, node_id: u32) -> Arc<Mutex<MemoryStorageServiceClient<InterceptedService<Channel, AuthInterceptor>>>> {
        {
            let storage_service_nodes = self.storage_service_nodes.lock().await;
            if let Some(client) = storage_service_nodes.get(&node_id) {
                return client.clone();
            }
        }

        let endpoint = self.controller_client.lock().await.get_memory_storage_node_by_id(GetMemoryStorageNodeByIdRequest {
            node_id: Some(StorageNodeId {
                id: node_id,
            }),
        }).await.unwrap().into_inner().endpoint;

        let client = Arc::new(Mutex::new(MemoryStorageServiceClient::with_interceptor(
            Endpoint::from_str(&endpoint).unwrap().connect().await.unwrap(), 
            AuthInterceptor::new(self.token.clone())
        )));

        self.storage_service_nodes.lock().await.insert(node_id, client.clone());

        client
    }
}