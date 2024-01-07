use {
    std::{process::Command, fs, path::Path},
    tracing::info,
    crate::thesis::{
        content::{
            Content,
            Language,
            thesis_content,
            thesis_content_for_plagiarism_check,
            thesis_application_code_listing,
            thesis_docx_template,
            topic_card_docx_template,
            documentation::documentation,
            practice_report_content,
            conference_abstract::{conference_abstract, conference_abstract_docx_template},
        },
        context::Context,
        engine::{Document, Block, TextBlockComponent, render_block_to_html, print_placeholders},
    },
    super::Args,
};

pub mod components;

pub mod content;
pub mod context;
pub mod engine;
pub mod utils;

pub fn build_thesis(args: &Args) {
    fs::create_dir_all("./output").unwrap();

    let content = Content::new();
    let mut documents = vec![
    ];

    if args.thesis {
        documents.push(
            Document::new("thesis", thesis_content(&content)).with_docx_template(thesis_docx_template(true)),
        );
    }

    if args.card {
        documents.push(
            Document::new("іп22мп_волобуєв_КАРТКА", Block::TopicCard).with_docx_template(topic_card_docx_template())
        );
    }

    if args.practice_report {
        documents.push(
            Document::new("іп22мп_волобуєв_звіт", practice_report_content(&content)).with_docx_template(thesis_docx_template(true)).with_prepend_pdf("./config/docs/практика_титулка звіту.pdf".to_owned()),
        );
    }

    if args.docs {
        documents.push(
            Document::new("documentation", documentation()),
        );
    }

    if args.conference_abstract {
        let content = conference_abstract(&if args.ukrainian {
            Language::Ukrainian
        } else {
            Language::English
        });

        documents.push(
            Document::new("methods_and_software_for_providing_software_defined_far_memory_in_distributed_systems", content).with_docx_template(conference_abstract_docx_template())
        );
    }

    if args.plagiarism_check_docs {
        documents.push(
            Document::new("plagiarism_check/ІП22мп_Волобуєв_ПЗ", thesis_content_for_plagiarism_check()).with_docx_template(thesis_docx_template(false)),
        );
        documents.push(
            Document::new("plagiarism_check/ІП22мп_Волобуєв_КОД", thesis_application_code_listing()).with_docx_template(thesis_docx_template(false)),
        );
    }

    for document in documents {
        let mut context = Context::new();

        let docx_path = format!("./output/{}.docx", document.name());

        let path = Path::new(&docx_path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let docx_file = fs::File::create(&docx_path).unwrap();

        info!("building {}", document.name());

        let document_content = document.content();
        print_placeholders(&document_content);

        if args.html {
            let html_path = format!("./output/{}/index.html", document.name());
            info!("generating {} to {:?}", document.name(), html_path);

            copy_images_to_output(&format!("./output/{}", document.name()), &document_content);

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

            Command::new("docx2pdf").args([&docx_path, &pdf_path]).output().unwrap();

            for pdf_to_prepend in document.prepend_pdf() {
                fs::rename(&pdf_path, "./output/tmp.pdf").unwrap();
                Command::new("pdftk").args([&pdf_to_prepend, "./output/tmp.pdf", "cat", "output", &pdf_path]).output().unwrap();
            }
        }
    }
}

fn copy_images_to_output(path: &str, block: &Block) {
    match block {
        Block::SectionHeader(_) => (),
        Block::SubsectionHeader(_) => (),
        Block::Paragraph(_) => (),
        Block::OrderedList(_) => (),
        Block::UnorderedList(_) => (),
        Block::Image(image) => {
            fs::create_dir_all(path).unwrap();
            let output_path = std::path::Path::new(path).join(image.path());
            let from = format!("./images/{}", image.path());
            if let Err(err) = fs::copy(&from, output_path) {
                panic!("failed to copy image from {:?} because {:?}", from, err);
            }
        },
        Block::Placeholder(inner, _) => copy_images_to_output(path, &*inner),
        Block::Multiple(inner) => inner.iter().for_each(|v| copy_images_to_output(path, v)),
        Block::ReferencesList(_) => (),
        Block::TableOfContents => (),
        Block::AbstractSection(_, _) => (),
        Block::TaskSection => (),
        Block::FrontPage => (),
        Block::TopicCard => (),
        Block::Note(_) => (),
        Block::Table(_) => (),
        Block::Application(_) => (),
    }
}
