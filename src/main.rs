use {
    std::{fs, process::exit},
    clap::Parser,
    tracing::{span, Level, info},
    crate::{
        utils::{init_logging, init_tracing, metrics::init_metrics, generate_run_id},
        thesis::build_thesis,
        storage::run_storage_server,
        manager::run_manager_node,
        client::run_replacement_policies_demo,
        demo::{
            llm_inference::run_llm_inference_demo,
            benchmark::run_benchmark,
            simple::run_simple_demo,
            block_device::run_block_device_demo,
        },
    },
};

mod client;
mod demo;
mod manager;
mod storage;
mod thesis;

mod utils;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    trace: bool,

    #[arg(long)]
    time_limit: Option<u64>,

    #[arg(long)]
    memory_limit_mb: Option<u64>,

    #[arg(long)]
    run_id: Option<String>,

    // components
    #[arg(long)]
    storage: bool,

    #[arg(long)]
    manager: bool,

    #[arg(long)]
    port: Option<u16>,

    #[arg(long)]
    manager_endpoint: Option<String>,

    #[arg(long)]
    storage_endpoint: Option<String>,

    // demo
    #[arg(long)]
    simple_demo: bool,

    #[arg(long)]
    llm_inference_demo: bool,

    #[arg(long)]
    benchmark: bool,

    #[arg(long)]
    optimize: bool,

    #[arg(long)]
    run_loop: bool, // run demo in a loop until it crashes, lol.

    #[arg(long)]
    block_device_demo: bool,

    #[arg(long)]
    replacement_policies_demo: bool,

    // thesis
    #[arg(long)]
    thesis: bool,

    #[arg(long)]
    card: bool,

    #[arg(long)]
    practice_report: bool,

    #[arg(long)]
    docs: bool,

    #[arg(long)]
    docx: bool,

    #[arg(long)]
    html: bool,

    #[arg(long)]
    pdf: bool,
}

pub fn main() {
    let args = Args::parse();

    let trace_guard = if args.trace {
        Some(init_tracing())
    } else {
        init_logging();
        None
    };

    ctrlc::set_handler(move || {
        if let Some(guard) = &trace_guard {
            guard.flush();
        }

        println!("stop.");

        exit(0);
    }).unwrap();

    if args.storage {
        let metrics = init_metrics(None);
        run_storage_server(metrics, read_token(), args.port);
    } else if args.simple_demo {
        run_simple_demo();
    } else if args.llm_inference_demo {
        let run_id = generate_run_id();
        let run_id = args.run_id.map(|prefix| format!("{}_{}", prefix, run_id)).unwrap_or(run_id);

        println!("run id: {:?}", run_id);
        let metrics = init_metrics(Some(run_id.clone()));

        let run = || {
            span!(Level::DEBUG, "llm_inference_demo")
                .in_scope(|| run_llm_inference_demo(
                    metrics.clone(),
                    run_id.clone(),
                    &read_token(),
                    args.storage_endpoint.clone().map(|v| v.split(",").map(|v| v.to_owned()).collect::<Vec<String>>()).unwrap_or(Vec::new()),
                    args.manager_endpoint.clone(),
                    args.time_limit.unwrap_or(10 * 60),
                    args.optimize,
                    args.memory_limit_mb.map(|v| v * 1024 * 1024)
                ));
        };

        if args.run_loop {
            info!("running in a loop");

            loop {
                run();
            }
        } else {
            run();
        }
    } else if args.benchmark {
        run_benchmark(&read_token(), args.storage_endpoint.clone());
    } else if args.block_device_demo {
        let run_id = generate_run_id();
        let run_id = args.run_id.map(|prefix| format!("{}_{}", prefix, run_id)).unwrap_or(run_id);

        println!("run id: {:?}", run_id);
        let metrics = init_metrics(Some(run_id.clone()));

        run_block_device_demo(
            metrics,
            run_id,
            &read_token(),
            args.storage_endpoint.clone().map(|v| v.split(",").map(|v| v.to_owned()).collect::<Vec<String>>()).unwrap_or(Vec::new()),
            args.memory_limit_mb.map(|v| v * 1024 * 1024)
        );
    } else if args.manager {
        run_manager_node(read_token());
    } else if args.replacement_policies_demo {
        run_replacement_policies_demo();
    } else if args.thesis || args.card || args.docs || args.practice_report {
        build_thesis(&args);
    }
}

fn read_token() -> String {
    fs::read_to_string("config/.token").unwrap().replace("\n", "")
}
