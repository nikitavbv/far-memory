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
        sections::{
            FrontPageSection, 
            TaskSection, 
            AbstractSection, 
            TableOfContentsSection,
            AbbreviationsListSection,
            IntroSection,
            MainSection,
            ConclusionsSection,
            ReferencesSection,
        },
        content::{Content, Language},
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
}

fn main() {
    init_logging();

    let args = Args::parse();

    let path = "./thesis.docx";
    let file = std::fs::File::create(path).unwrap();

    info!("generating thesis to {:?}", path);

    let mut context = Context::new();
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
        .add_style(Style::new("Heading1", StyleType::Paragraph).name("Heading 1").bold())
        .add_style(Style::new("Heading2", StyleType::Paragraph).name("Heading 2").bold())
        .add_front_page_section(&content)
        .add_task_section(&mut context, &content)
        .add_abstract_section(&mut context, &content, &Language::Ukrainian)
        .add_abstract_section(&mut context, &content, &Language::English)
        .add_table_of_contents_section()
        .add_abbreviations_list_section()
        .add_intro_section(&mut context)
        .add_main_section(&mut context)
        .add_conclusions_section()
        .add_references_section(&mut context)
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
