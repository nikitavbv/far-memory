use {
    tracing::info,
    crate::client::{FarMemoryClient, FarMemoryVec, backend::disk::LocalDiskBackend},
};

pub fn run_simple_demo() {
    info!("running a simple demo");

    let client = FarMemoryClient::new(Box::new(LocalDiskBackend::new()), 1000);
    let vec = FarMemoryVec::from_vec(client, vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02]);

    let res = vec.to_local_vec();
    println!("res: {:?}", res);
}