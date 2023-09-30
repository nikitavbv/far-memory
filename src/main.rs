use {
    tracing::info,
    clap::Parser,
    crate::{
        utils::init_logging,
        config::Config,
        client::{run_block_storage_client, test_mode::run_test_mode},
        memory_storage::run_memory_storage_server,
        controller::run_controller_server,
        thesis::build_thesis,
    },
};

mod client;
mod thesis;

mod config;
mod controller;
mod memory_storage;
mod rpc;
mod utils;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    thesis: bool,

    #[arg(short, long)]
    card: bool,

    #[arg(short, long)]
    docs: bool,

    #[arg(short, long)]
    docx: bool,

    #[arg(short, long)]
    html: bool,

    #[arg(short, long)]
    pdf: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logging();
    let args = Args::parse();

    if args.thesis || args.card || args.docs {
        let x = 42;
        build_thesis(&args);
    } else {
        info!("running far-memory");

        let config = Config::load();
        let far_memory_block_size = 2 * 1024 * 1024;

        if config.controller_enabled() {
            run_controller_server(config.access_token(), config.controller_storage_nodes()).await;
        }

        if config.memory_storage_enabled() {
            run_memory_storage_server(config.access_token(), far_memory_block_size).await;
        }

        if config.block_storage_client_enabled() {
            run_block_storage_client(config.endpoint(), config.access_token(), far_memory_block_size).await;
        }

        if config.test_mode_enabled() {
            run_test_mode(config.endpoint(), config.access_token(), far_memory_block_size).await;
        }
    }

    Ok(())
}
