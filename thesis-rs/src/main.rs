use {
    std::process::Command,
    clap::Parser,
    tracing::info,
    docx_rs::{
        Docx,
        PageMargin, 
        RunFonts,
        Style,
        StyleType,
    },
    crate::{
        documents::{ThesisDocument, TopicCardDocument},
        content::Content,
        context::Context,
        utils::init_logging,
    },
};

pub mod components;
pub mod documents;
pub mod sections;

pub mod content;
pub mod context;
pub mod utils;

#[derive(Parser, Debug)]
struct Args {   
    #[arg(short, long)]
    pdf: bool,
    #[arg(short, long)]
    open: bool,
}

fn main() {
    init_logging();

    let args = Args::parse();

    let mut context = Context::new();
    let content = Content::new();

    let path = std::fs::File::create("./output/іп22мп_волобуєв_КАРТКА.docx").unwrap();
    Docx::new()
        .page_margin(
            PageMargin::new()
                .left(mm_to_twentieth_of_a_point(10.0))
                .top(mm_to_twentieth_of_a_point(10.0))
                .bottom(mm_to_twentieth_of_a_point(9.6))
                .right(mm_to_twentieth_of_a_point(9.7))   
        )
        .default_fonts(RunFonts::new().cs("Arial").hi_ansi("Arial"))
        .default_size(2 * 11)
        .default_tab_stop(0)
        .add_topic_card_document(&content)
        .build()
        .pack(&path)
        .unwrap();

    let path = "./output/thesis.docx";
    let file = std::fs::File::create(path).unwrap();
    info!("generating thesis to {:?}", path);
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
        .add_style(Style::new("Heading1", StyleType::Paragraph).name("Heading 1").bold())
        .add_style(Style::new("Heading2", StyleType::Paragraph).name("Heading 2").bold())
        .add_thesis_document(&mut context, &content)
        .build()
        .pack(&file)
        .unwrap();

    if args.pdf {
        info!("converting to pdf");
        Command::new("docx2pdf").args(["./output/thesis.docx", "./output/thesis.pdf"]).output().unwrap();
    
        if args.open {
            info!("done, opening resulting file");
            Command::new("open").args(["./thesis.pdf"]).output().unwrap();
        }
    }
}

fn mm_to_twentieth_of_a_point(mm: f32) -> i32 {
    (mm * 56.6929133858).round() as i32
}
