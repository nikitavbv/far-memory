use {
    std::{sync::Arc, str::FromStr},
    tokio::sync::Mutex,
    tonic::{codegen::InterceptedService, transport::{Endpoint, Channel}},
    crate::{
        utils::AuthInterceptor,
        rpc::{
            controller_service_client::ControllerServiceClient,
            ControllerAllocateMemoryBlockRequest,
        },
    },
    super::block_map::RemoteBlockId,
};

#[derive(Clone)]
pub struct ControllerClient {
    client: Arc<Mutex<ControllerServiceClient<InterceptedService<Channel, AuthInterceptor>>>>,
}

impl ControllerClient {
    pub async fn new(endpoint: String, token: String) -> Self {
        Self {
            client: Arc::new(Mutex::new(ControllerServiceClient::with_interceptor(
                Endpoint::from_str(&endpoint).unwrap().connect().await.unwrap(), 
                AuthInterceptor::new(token)
            ))),
        }
    }

    pub async fn allocate_block(&self) -> RemoteBlockId {
        let res = self.client.lock().await.controller_allocate_memory_block(ControllerAllocateMemoryBlockRequest {
        }).await.unwrap().into_inner();

        RemoteBlockId::new(res.node_id.unwrap().id, res.block_id.unwrap().id)
    }
}