use crate::{
    utils::init_logging,
    memory_storage::run_memory_storage_server,
};

pub mod memory_storage;
pub mod rpc;
pub mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logging();

    run_memory_storage_server().await;

    Ok(())
}
