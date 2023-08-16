use {
    std::str::FromStr,
    tracing::info,
    tokio::sync::Mutex,
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
            StorageNodeId,
            ControllerAllocateMemoryBlockRequest,
            ControllerAllocateMemoryBlockResponse,
            AllocateMemoryBlockRequest,
        },
    },
};

pub async fn run_controller_server(access_token: String, storage_nodes_config: Vec<StorageServerConfig>) {
    info!("running controller server");

    let addr = "0.0.0.0:9000".parse().unwrap();

    let mut nodes = Vec::new();
    for config in &storage_nodes_config {
        nodes.push(StorageNode::new(nodes.len() as u32, config.endpoint(), access_token.clone()).await);
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
        let mut node_with_lowest_blocks = None;
        let mut lowest_blocks_count = None;
        for node in &self.nodes {
            let blocks_count = node.total_blocks_allocated().await;
            if lowest_blocks_count.is_none() || lowest_blocks_count.unwrap() > blocks_count {
                lowest_blocks_count = Some(blocks_count);
                node_with_lowest_blocks = Some(node);
            }
        }

        // call allocate memory block on that node
        let block_id = node_with_lowest_blocks.unwrap().allocate_memory_block().await;

        // return block id and node id
        Ok(Response::new(ControllerAllocateMemoryBlockResponse {
            node_id: Some(StorageNodeId {
                id: node_with_lowest_blocks.unwrap().id(),
            }),
            block_id: Some(block_id),
        }))
    } 
}

struct StorageNode {
    id: u32,
    client: Mutex<MemoryStorageServiceClient<InterceptedService<Channel, AuthInterceptor>>>,
    blocks_allocated: Mutex<Vec<MemoryBlockId>>,
}

impl StorageNode {
    pub async fn new(id: u32, endpoint: String, token: String) -> Self {
        Self {
            id,
            client: Mutex::new(MemoryStorageServiceClient::with_interceptor(
                Endpoint::from_str(&endpoint).unwrap().connect().await.unwrap(),
                AuthInterceptor::new(token)
            )),
            blocks_allocated: Mutex::new(Vec::new()),
        }
    }

    pub async fn allocate_memory_block(&self) -> MemoryBlockId {
        let res = self.client.lock().await.allocate_memory_block(AllocateMemoryBlockRequest {}).await.unwrap().into_inner();
        let id = res.id.as_ref().cloned().unwrap();
        
        self.blocks_allocated.lock().await.push(id.clone());

        id
    }

    pub async fn total_blocks_allocated(&self) -> u32 {
        self.blocks_allocated.lock().await.len() as u32
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}