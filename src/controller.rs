use {
    std::str::FromStr,
    tracing::info,
    tonic::{transport::{Server, Endpoint, Channel}, Request, Response, Status, codegen::InterceptedService},
    crate::{
        config::StorageServerConfig,
        utils::AuthInterceptor,
        rpc::{
            memory_storage_service_client::MemoryStorageServiceClient,
            controller_service_server::{
                ControllerService,
                ControllerServiceServer,
            },
            MemoryBlockId,
            ControllerAllocateMemoryBlockRequest,
            ControllerAllocateMemoryBlockResponse,
        },
    },
};

pub async fn run_controller_server(access_token: String, storage_nodes_config: Vec<StorageServerConfig>) {
    info!("running controller server");

    let addr = "0.0.0.0:9000".parse().unwrap();

    let mut nodes = Vec::new();
    for config in &storage_nodes_config {
        nodes.push(StorageNode::new(config.endpoint(), access_token.clone()).await);
    }

    Server::builder()
        .add_service(ControllerServiceServer::new(ControllerServiceHandler::new(access_token, nodes)))
        .serve(addr)
        .await
        .unwrap();
}

struct ControllerServiceHandler {
    access_token: String,
    nodes: Vec<StorageNode>,
}

impl ControllerServiceHandler {
    pub fn new(access_token: String, nodes: Vec<StorageNode>) -> Self {
        Self {
            access_token,
            nodes,
        }
    }

    fn check_auth<T>(&self, request: &Request<T>) -> Result<(), Status> {
        let headers = request.metadata().clone().into_headers();
        let token = headers.get("x-access-token").unwrap().to_str().unwrap();

        if token == &self.access_token {
            Ok(())
        } else {
            Err(Status::unauthenticated("invalid access token"))
        }
    }
}

#[tonic::async_trait]
impl ControllerService for ControllerServiceHandler {
    async fn controller_allocate_memory_block(&self, req: Request<ControllerAllocateMemoryBlockRequest>) -> Result<Response<ControllerAllocateMemoryBlockResponse>, Status> {
        self.check_auth(&req)?;
        
        // pick node with lowest number of blocks allocated

        // call allocate memory block on that node
        // return block id and node id

        unimplemented!()
    } 
}

struct StorageNode {
    client: MemoryStorageServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    blocks_allocated: Vec<MemoryBlockId>,
}

impl StorageNode {
    pub async fn new(endpoint: String, token: String) -> Self {
        Self {
            client: MemoryStorageServiceClient::with_interceptor(
                Endpoint::from_str(&endpoint).unwrap().connect().await.unwrap(),
                AuthInterceptor::new(token)
            ),
            blocks_allocated: Vec::new(),
        }
    }

    pub async fn allocate_memory_block(&self) -> MemoryBlockId {
        // TODO: call client to allocate block, save it to blocks_allocated and return id
        unimplemented!()
    }
}