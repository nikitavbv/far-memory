use {
    std::{io::Error, str::FromStr, collections::HashMap},
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
        MemoryBlockId,
    },
};

pub async fn run_block_storage_client(endpoint: String, token: String, far_memory_block_size: u64) {
    info!("starting block storage client");

    let mut device = FarMemoryDevice::new(endpoint, token, far_memory_block_size).await;

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

    far_memory_block_size: u64,
    blocks_initialized: HashMap<u64, MemoryBlockId>,

    data: Vec<u8>,
}

impl FarMemoryDevice {
    pub async fn new(endpoint: String, token: String, far_memory_block_size: u64) -> Self {
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

            far_memory_block_size,
            blocks_initialized: HashMap::new(),

            data: vec![0; 1024 * 1024 * 1024],
        }
    }

    fn block_for_offset(&self, offset: u64) -> u64 {
        offset / self.far_memory_block_size
    }

    fn block_id_for_block_offset(&mut self, block_offset: u64) -> MemoryBlockId {
        if let Some(id) = self.blocks_initialized.get(&block_offset) {
            return id.clone();
        }

        let res = self.runtime.block_on(async {
            self.client.allocate_memory_block(AllocateMemoryBlockRequest {}).await.unwrap().into_inner()
        });

        info!("initialized block: {:?}", res);

        let id = res.id.unwrap();
        self.blocks_initialized.insert(block_offset, id.clone());

        id
    }
}

impl BlockDevice for FarMemoryDevice {
    fn read(&mut self, offset: u64, bytes: &mut [u8]) -> Result<(), Error> {
        let begin_block_index = self.block_for_offset(offset);
        let end_block_index = self.block_for_offset(offset + bytes.len() as u64);

        for block in begin_block_index..end_block_index+1 {
            let block_id = self.block_id_for_block_offset(block);
            info!("block id: {:?}", block_id);
        }

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
