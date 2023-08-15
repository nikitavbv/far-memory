use {
    tracing::info,
    crate::{
        utils::init_logging,
        config::Config,
        client::run_block_storage_client,
        memory_storage::run_memory_storage_server,
        controller::run_controller_server,
    },
};

pub mod client;
pub mod config;
pub mod controller;
pub mod memory_storage;
pub mod rpc;
pub mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logging();

    info!("running far-memory");

    let config = Config::load();
    let far_memory_block_size = 2 * 1024 * 1024;

    if config.memory_storage_enabled() {
        run_memory_storage_server(config.access_token(), far_memory_block_size).await;
    }

    if config.block_storage_client_enabled() {
        run_block_storage_client(config.endpoint(), config.access_token(), far_memory_block_size).await;
    }

    if config.controller_enabled() {
        run_controller_server(config.access_token(), config.controller_storage_nodes()).await;
    }

    Ok(())
}
