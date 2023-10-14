use {
    std::fs,
    clap::Parser,
    tracing::{span, Level},
    crate::{
        utils::{init_logging, init_tracing},
        thesis::build_thesis,
        storage::run_storage_server,
        demo::{
            llm_inference::run_llm_inference_demo,
            benchmark::run_benchmark,
            simple::run_simple_demo,
        },
    },
};

mod client;
mod demo;
mod storage;
mod thesis;

mod utils;

#[derive(Parser, Debug)]
pub struct Args {
    // global settings
    #[arg(long)]
    trace: bool,

    // storage
    #[arg(long)]
    storage: bool,
    
    #[arg(long)]
    storage_endpoint: Option<String>,

    // demo
    #[arg(long)]
    simple_demo: bool,

    #[arg(long)]
    llm_inference_demo: bool,

    #[arg(long)]
    benchmark: bool,

    // thesis
    #[arg(long)]
    thesis: bool,

    #[arg(long)]
    card: bool,

    #[arg(long)]
    docs: bool,

    #[arg(long)]
    docx: bool,

    #[arg(long)]
    html: bool,

    #[arg(long)]
    pdf: bool,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let guard = if args.trace {
        Some(init_tracing())
    } else {
        init_logging();
        None
    };

    ctrlc::set_handler(move || {
        if guard.is_some() {
            guard.as_ref().unwrap().flush();
        }

        println!("stop.");
        
        std::process::exit(0);
    }).unwrap();

    if args.storage {
        run_storage_server(read_token());
    } else if args.simple_demo {
        run_simple_demo();
    } else if args.llm_inference_demo {
        span!(Level::DEBUG, "llm_inference_demo").in_scope(|| run_llm_inference_demo(&read_token(), &args.storage_endpoint.unwrap()))
    } else if args.benchmark {
        run_benchmark(&read_token(), &args.storage_endpoint.unwrap());
    } else if args.thesis || args.card || args.docs {
        build_thesis(&args);
    }

    Ok(())
}

fn read_token() -> String {
    fs::read_to_string(".token").unwrap().replace("\n", "")
}
