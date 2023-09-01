use {
    std::process::Command,
    clap::Parser,
    tracing::info,
    docx_rs::{
        Docx,
        PageMargin, 
        RunFonts, 
        AbstractNumbering,
        Level,
        Start,
        NumberFormat,
        LevelText,
        LevelJc,
    },
    crate::{
        sections::{FrontPageSection, TaskSection},
        content::Content,
        utils::init_logging,
    },
};

pub mod components;
pub mod sections;

pub mod content;
pub mod utils;

#[derive(Parser, Debug)]
struct Args {   
    #[arg(short, long)]
    pdf: bool,
}

fn main() {
    init_logging();

    let args = Args::parse();

    let path = "./thesis.docx";
    let file = std::fs::File::create(path).unwrap();

    info!("generating thesis to {:?}", path);

    let content = Content::new();

    Docx::new()
        .page_margin(
            PageMargin::new()
                .left(mm_to_twentieth_of_a_point(30.0))
                .top(mm_to_twentieth_of_a_point(20.0))
                .bottom(mm_to_twentieth_of_a_point(20.0))
                .right(mm_to_twentieth_of_a_point(10.0))
        )
        .default_fonts(RunFonts::new().cs("Times New Roman"))
        .default_size(28) // 14
        .default_tab_stop(0)
        .add_abstract_numbering(
            AbstractNumbering::new(1)
                .add_level(Level::new(
                    0,
                    Start::new(1),
                    NumberFormat::new("decimal"),
                    LevelText::new("%1. "),
                    LevelJc::new("start")
                )
            )
        )
        .add_front_page_section(&content)
        .add_task_section(&content)
        .build()
        .pack(file)
        .unwrap();

    if args.pdf {
        info!("converting to pdf");
        Command::new("docx2pdf").args(["./thesis.docx", "./thesis.pdf"]).output().unwrap();
    
        info!("done, opening resulting file");
        Command::new("open").args(["./thesis.pdf"]).output().unwrap();
    }
}

fn mm_to_twentieth_of_a_point(mm: f32) -> i32 {
    (mm * 56.6929133858).round() as i32
}
