use crate::{
    utils::init_logging,
    client::run_block_storage_client,
    memory_storage::run_memory_storage_server,
};

pub mod client;
pub mod memory_storage;
pub mod rpc;
pub mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logging();

    run_block_storage_client().await;
    // run_memory_storage_server().await;

    Ok(())
}
