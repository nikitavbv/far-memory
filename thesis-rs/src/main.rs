use {
    std::{process::Command, fs},
    clap::Parser,
    tracing::info,
    crate::{
        content::{Content, thesis_content, thesis_docx_template, topic_card_docx_template},
        context::Context,
        utils::init_logging,
        engine::{Document, Block, TextBlockComponent},
    },
};

pub mod components;

pub mod content;
pub mod context;
pub mod engine;
pub mod utils;

#[derive(Parser, Debug)]
struct Args {   
    #[arg(short, long)]
    card: bool,

    #[arg(short, long)]
    pdf: bool,
    #[arg(short, long)]
    open: bool,
}

fn main() {
    init_logging();

    let args = Args::parse();

    fs::create_dir_all("./output").unwrap();

    let content = Content::new();
    let mut documents = vec![
        Document::new("thesis", thesis_content()).with_docx_template(thesis_docx_template()),
    ];

    if args.card {
        documents.push(
            Document::new("іп22мп_волобуєв_КАРТКА", Block::TopicCard).with_docx_template(topic_card_docx_template())
        );
    }

    for document in documents {
        let mut context = Context::new();

        let docx_path = format!("./output/{}.docx", document.name());
        let docx_file = fs::File::create(&docx_path).unwrap();
        info!("generating {} to {:?}", document.name(), docx_file);

        document.docx()
            .add_text_block(&mut context, &content, document.content())
            .build()
            .pack(docx_file)
            .unwrap();

        if args.pdf {
            info!("converting {} to pdf", document.name());
            let pdf_path = format!("./output/{}.pdf", document.name());

            Command::new("docx2pdf").args([docx_path, pdf_path]).output().unwrap();
        }
    }

}