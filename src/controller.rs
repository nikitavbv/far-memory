use {
    tracing::info,
    tonic::{transport::Server, Request, Response, Status},
    crate::rpc::{
        controller_service_server::{
            ControllerService,
            ControllerServiceServer,
        },
        ControllerAllocateMemoryBlockRequest,
        ControllerAllocateMemoryBlockResponse,
    },
};

pub async fn run_controller_server(access_token: String) {
    info!("running controller server");

    let addr = "0.0.0.0:9000".parse().unwrap();

    Server::builder()
        .add_service(ControllerServiceServer::new(ControllerServiceHandler::new(access_token)))
        .serve(addr)
        .await
        .unwrap();
}

struct ControllerServiceHandler {
    access_token: String,
}

impl ControllerServiceHandler {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
        }
    }
}

#[tonic::async_trait]
impl ControllerService for ControllerServiceHandler {
    async fn controller_allocate_memory_block(&self, req: Request<ControllerAllocateMemoryBlockRequest>) -> Result<Response<ControllerAllocateMemoryBlockResponse>, Status> {
        unimplemented!()
    } 
}