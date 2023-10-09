use {
    tracing::info,
    crate::client::{FarMemoryBuffer, FarMemoryClient},
};

pub fn run_simple_demo() {
    info!("running a simple demo");

    let client = FarMemoryClient::new();
    let buffer = FarMemoryBuffer::from_bytes(client, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);    
}