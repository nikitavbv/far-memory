use {
    tracing::{info, warn},
    vblk::{mount, BlockDevice},
    prometheus::Registry,
    crate::client::{
        FarMemoryBackend,
        NetworkNodeBackend,
        ErasureCodingBackend,
        ReplicationBackend,
        LocalDiskBackend,
        InstrumentedBackend,
        FarMemoryClient,
        FarMemoryBuffer,
    },
};

pub fn run_block_device_demo(metrics: Registry, run_id: String, token: &str, endpoints: Vec<String>, local_max_memory: Option<u64>) {
    info!("running block device demo");
    
    let backend: Box<dyn FarMemoryBackend> = if !endpoints.is_empty() {
        if endpoints.len() == 1 {
            info!("running in single backend node mode");
            Box::new(NetworkNodeBackend::new(&endpoints[0], token, run_id))
        } else if endpoints.len() == 5 {
            info!("running in erasure coded mode");

            let nodes: Vec<_> = endpoints.iter()
                .map(|v| Box::new(NetworkNodeBackend::new(&v, token, run_id.clone())) as Box<dyn FarMemoryBackend>)
                .collect();

            Box::new(ErasureCodingBackend::new(nodes))
        } else {
            let nodes: Vec<_> = endpoints.iter()
                .map(|v| Box::new(NetworkNodeBackend::new(&v, token, run_id.clone())) as Box<dyn FarMemoryBackend>)
                .collect();

            info!("running in replication mode with {} nodes", nodes.len());

            Box::new(ReplicationBackend::new(nodes))
        }
    } else {
        warn!("no storage endpoint provided, falling back to disk backend");
        Box::new(LocalDiskBackend::new())
    };
    
    let backend = Box::new(InstrumentedBackend::new(metrics.clone(), backend));
    let mut client = FarMemoryClient::new(backend, local_max_memory.unwrap_or(25000 * 1024 * 1024));
    client.track_metrics(metrics.clone());
    client.start_swap_out_thread();
    
    let mut device = FarMemoryDevice::new(client, 30000 * 1024 * 1024);
    unsafe {
        mount(&mut device, "/dev/nbd1", |_device| Ok(())).unwrap();
    }
}

struct FarMemoryDevice {
    buffer: FarMemoryBuffer,
    size: u64,
}

impl FarMemoryDevice {
    pub fn new(client: FarMemoryClient, size: u64) -> Self {
        Self {
            buffer: FarMemoryBuffer::zeros_with_span_size(client, size, 100 * 1024 * 1024),
            size,
        }
    }
}

impl BlockDevice for FarMemoryDevice {
    fn read(&mut self, offset: u64, bytes: &mut [u8]) -> std::io::Result<()> {
        let offset = offset as usize;
        let data = self.buffer.slice(offset..offset + bytes.len());
        bytes.copy_from_slice(&data);
        Ok(())
    }
    
    fn write(&mut self, offset: u64, bytes: &[u8]) -> std::io::Result<()> {
        self.buffer.write_range(offset as usize, bytes);
        Ok(())
    }
    
    fn block_size(&self) -> u32 {
        1024
    }
    
    fn blocks(&self) -> u64 {
        self.size / self.block_size() as u64
    }
}
