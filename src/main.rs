use {
    clap::Parser,
    crate::{
        utils::init_logging,
        thesis::build_thesis,
        demo::{
            llm_inference::run_llm_inference_demo,
            simple::run_simple_demo,
        },
    },
};

mod client;
mod demo;
mod thesis;

mod utils;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    simple_demo: bool,

    #[arg(long)]
    llm_inference_demo: bool,

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
    init_logging();
    let args = Args::parse();

    if args.simple_demo {
        run_simple_demo();
    } else if args.llm_inference_demo {
        run_llm_inference_demo();
    } else if args.thesis || args.card || args.docs {
        build_thesis(&args);
    }

    Ok(())
}
