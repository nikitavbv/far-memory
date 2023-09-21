use {
    std::{process::Command, fs},
    clap::Parser,
    tracing::info,
    crate::{
        content::{Content, thesis_content, thesis_docx_template, topic_card_docx_template},
        context::Context,
        utils::init_logging,
        engine::{Document, Block, TextBlockComponent, render_block_to_html, print_placeholders},
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
    docx: bool,

    #[arg(short, long)]
    html: bool,

    #[arg(short, long)]
    pdf: bool,
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

        info!("building {}", document.name());

        let document_content = document.content();
        print_placeholders(&document_content);

        if args.html {
            let html_path = format!("./output/{}.html", document.name());
            info!("generating {} to {:?}", document.name(), html_path);

            let html = render_block_to_html(document_content.clone());
            fs::write(html_path, html).unwrap();
        }

        if args.docx || args.pdf {
            info!("generating {} to {:?}", document.name(), docx_path);

            document.docx()
                .add_text_block(&mut context, &content, document_content)
                .build()
                .pack(docx_file)
                .unwrap();
        }

        if args.pdf {
            info!("converting {} to pdf", document.name());
            let pdf_path = format!("./output/{}.pdf", document.name());

            Command::new("docx2pdf").args([docx_path, pdf_path]).output().unwrap();
        }
    }
}
