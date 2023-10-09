use {
    tracing::info,
    crate::client::{FarMemoryBuffer, FarMemoryClient, backend::disk::LocalDiskBackend},
};

pub fn run_simple_demo() {
    info!("running a simple demo");

    let client = FarMemoryClient::new(Box::new(LocalDiskBackend::new()));
    let buffer = FarMemoryBuffer::from_bytes(client, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    buffer.swap_out();

    let x = buffer.slice(3..6);

    println!("x is: {:?}", x);
}