use std::str::FromStr;

use {
    std::io::Error,
    tracing::info,
    vblk::{mount, BlockDevice},
    tonic::{
        service::Interceptor,
        codegen::InterceptedService,
        transport::{Endpoint, Channel},
        metadata::MetadataValue,
    },
    crate::rpc::{
        memory_storage_service_client::MemoryStorageServiceClient,
        AllocateMemoryBlockRequest,
    },
};

pub async fn run_block_storage_client(endpoint: String, token: String) {
    info!("starting block storage client");

    let mut device = FarMemoryDevice::new(endpoint, token).await;

    tokio::task::spawn_blocking(move || {
        unsafe {
            mount(&mut device, "/dev/nbd1", |_device| Ok(())).unwrap();
        }
    });
}

struct AuthInterceptor {
    token: String,
}

impl AuthInterceptor {
    pub fn new(token: String) -> Self {
        Self {
            token,
        }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        request.metadata_mut().append("x-access-token", MetadataValue::try_from(&self.token).unwrap());
        Ok(request)
    }
}

struct FarMemoryDevice {
    runtime: tokio::runtime::Runtime,
    client: MemoryStorageServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    data: Vec<u8>,
}

impl FarMemoryDevice {
    pub async fn new(endpoint: String, token: String) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let client = runtime.spawn(async move {
            MemoryStorageServiceClient::with_interceptor(
                Endpoint::from_str(&endpoint).unwrap().connect().await.unwrap(), 
                AuthInterceptor::new(token)
            )
        }).await.unwrap();

        Self {
            runtime,
            client,
            data: vec![0; 1024 * 1024 * 1024],
        }
    }
}

impl BlockDevice for FarMemoryDevice {
    fn read(&mut self, offset: u64, bytes: &mut [u8]) -> Result<(), Error> {
        self.runtime.block_on(async {
            let res = self.client.allocate_memory_block(AllocateMemoryBlockRequest {}).await.unwrap();
            info!("res: {:?}", res);
        });
        
        bytes.copy_from_slice(&self.data[offset as usize..offset as usize + bytes.len()]);
        Ok(())
    }

    fn write(&mut self, offset: u64, bytes: &[u8]) -> std::io::Result<()> {
        self.data[offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }

    fn block_size(&self) -> u32 {
        1024
    }

    fn blocks(&self) -> u64 {
        100
    }
}
