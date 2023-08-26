use {
    std::{sync::Arc, str::FromStr},
    tokio::sync::Mutex,
    tonic::{codegen::InterceptedService, transport::{Endpoint, Channel}},
    crate::{
        utils::AuthInterceptor,
        rpc::controller_service_client::ControllerServiceClient,
    },
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
}