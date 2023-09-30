use {
    std::{process::Command, fs::File, io::ErrorKind},
    tracing::warn,
    docx_rs::{
        Docx, 
        Paragraph, 
        Tab, 
        LineSpacing, 
        Run, 
        AbstractNumbering, 
        Level, 
        Start, 
        NumberFormat, 
        LevelText, 
        LevelJc, 
        SpecialIndentType, 
        NumberingId, 
        AlignmentType, 
        Numbering, 
        IndentLevel,
        TableOfContents,
        TabLeaderType,
    },
    thiserror::Error,
    crate::thesis::{
        context::Context,
        content::{Content, Language, AbstractContent},
        components::{
            SectionHeaderComponent,
            ParagraphComponent, 
            UnorderedListComponent, 
            ImageComponent, 
            AbstractSection, 
            TaskSection,
            FrontPageSection, 
            TopicCardDocument,
        },
    },
};

#[derive(Error, Debug)]
pub enum PageCountingError {
    #[error("No pdf converter installed")]
    NoPdfConverterInstalled,
}

#[derive(Debug, Clone)]
pub enum Block {
    SectionHeader(String),
    SubsectionHeader(String),
    Paragraph(String),
    UnorderedList(Vec<String>),
    Image(ImageBlock),
    Placeholder(Box<Block>, String),
    Multiple(Vec<Block>),
    ReferencesList(Vec<String>),
    TableOfContents,
    AbstractSection(Language, AbstractContent),
    TaskSection,
    FrontPage,
    TopicCard,
    Note(String),
}

#[derive(Debug, Clone)]
pub struct ImageBlock {
    path: String,
    description: String,
}

impl ImageBlock {
    pub fn new(path: String, description: String) -> Self {
        Self {
            path,
            description,
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

pub fn render_block_to_docx(document: Docx, context: &mut Context, content: &Content, block: Block) -> Docx {
    render_block_to_docx_with_params(document, context, content, None, block)
}

fn render_block_to_docx_with_params(document: Docx, context: &mut Context, content: &Content, placeholder: Option<String>, block: Block) -> Docx {
    match block {
        Block::SectionHeader(text) => {
            let text = format!("{}   {}", context.next_section_index(), text);

            match placeholder {
                Some(v) => document.add_section_header_placeholder_component(text, v),
                None => document.add_section_header_component(text),
            }
        },
        Block::SubsectionHeader(text) => {
            let subsection_index = context.next_subsection_index(context.last_section_index());

            document.add_paragraph(
                Paragraph::new()
                    .add_tab(Tab::new().pos(710))
                    .line_spacing(LineSpacing::new().before(300).line(24 * 15))
                    .style("Heading2")
                    .add_run(Run::new().add_tab().add_text(format!("{}.{}   {}", context.last_section_index(), subsection_index, text)))
            )
        },
        Block::Paragraph(text) => match placeholder {
            Some(v) => document.add_paragraph_placeholder_component(text, v),
            None => document.add_paragraph_component(text),
        },
        Block::UnorderedList(list) => document.add_unordered_list_component(context, list),
        Block::Image(image) => document.add_image_component(context, context.last_section_index(), &image.path(), &image.description()),
        Block::Placeholder(inner, description) => render_block_to_docx_with_params(document, context, content, Some(description), *inner),
        Block::Multiple(blocks) => blocks.into_iter().fold(document, |doc, block| render_block_to_docx_with_params(doc, context, content, placeholder.clone(), block)),
        Block::ReferencesList(references) => {
            let numbering = context.next_numbering_id();

            let document = document
                .add_abstract_numbering(
                    AbstractNumbering::new(numbering)
                        .add_level(Level::new(
                            0,
                            Start::new(1),
                            NumberFormat::new("decimal"),
                            LevelText::new("%1. "),
                            LevelJc::new("start")
                        ).indent(None, Some(SpecialIndentType::Hanging(300)), None, None)
                    )
                )
                .add_numbering(Numbering::new(numbering, numbering));

            references.into_iter().fold(document, |document, reference| document.add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().line(24 * 15))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text(reference))))
        },
        Block::TableOfContents => document.add_table_of_contents(TableOfContents::new()
            .heading_styles_range(1, 3)
            .tab_leader_type(Some(TabLeaderType::None))
            .auto()
        ),
        Block::AbstractSection(language, abstract_content) => document.add_abstract_section(context, content, &abstract_content, &language),
        Block::TaskSection => document.add_task_section(context, content),
        Block::FrontPage => document.add_front_page_section(content),
        Block::TopicCard => document.add_topic_card_document(context, content),
        Block::Note(_) => panic!("note block is not supported in docx"),
    }
}

pub fn render_block_to_html(block: Block) -> String {
    format!(
        r#"
        <html>
            <head>
                <meta charset="utf-8" />
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css" integrity="sha384-X38yfunGUhNzHpBaEBsWLO+A0HDYOQi8ufWDkZ0k9e0eXz/tH3II7uKZ9msv++Ls" crossorigin="anonymous">
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=Open+Sans&display=swap" rel="stylesheet">
                <script type="text/javascript" src="https://livejs.com/live.js"></script>
                <style>
                    body {{
                        max-width: 768px;
                        margin: 0 auto;
                        background: #eee;
                        font-family: 'Open Sans', sans-serif;
                        line-height: 1.6;
                    }}

                    h1, h2 {{
                        font-weight: 400;
                        margin: 1em 0 0 0;
                    }}

                    h2 {{
                        color: #4b4b4b;
                    }}

                    .note {{
                        background: #1f8dd6;
                        border-radius: 3px;
                        color: #fff;
                        padding: 0.3em 1em;
                    }}

                    p {{
                        margin: 0;
                    }}
                </style>
            </head>

            <body>{}</body>
        </html>
        "#,
        render_block_to_html_inner(block)
    )
}

fn render_block_to_html_inner(block: Block) -> String {
    match block {
        Block::SectionHeader(text) => format!("<h1>{}</h1>", html_escape::encode_text(&text)),
        Block::SubsectionHeader(text) => format!("<h2>{}</h2>", html_escape::encode_text(&text)),
        Block::Paragraph(text) => format!("<p>{}</p>", html_escape::encode_text(&text)),
        Block::UnorderedList(text) => format!("<ul>{}</ul>", text.iter().map(|v| format!("<li>{}</li>", html_escape::encode_text(&v))).collect::<String>()),
        Block::Image(image) => format!("<img src=\"{}\" />", image.path()),
        Block::Placeholder(inner, _text) => format!("<div style=\"background-color: yellow;\">{}</div>", render_block_to_html_inner(*inner)),
        Block::Multiple(blocks) => blocks.into_iter().map(render_block_to_html_inner).collect::<String>(),
        Block::Note(text) => format!("<div class=\"note\">{}</div>", html_escape::encode_text(&text)),
        other => format!("<div>block of this type is not supported: {:?}</div>", other),
    }
}

pub fn subsection_header(text: impl Into<String>) -> Block {
    Block::SubsectionHeader(text.into())
}

pub fn paragraph(text: impl Into<String>) -> Block {
    Block::Paragraph(text.into())
}

pub fn unordered_list(list: Vec<String>) -> Block {
    Block::UnorderedList(list)
}

pub trait TextBlockComponent {
    fn add_text_block(self, context: &mut Context, content: &Content, block: Block) -> Self;
}

impl TextBlockComponent for Docx {
    fn add_text_block(self, context: &mut Context, content: &Content, block: Block) -> Self {
        render_block_to_docx(self, context, content, block)
    }
}

pub struct Document {
    name: String,
    content: Block,

    docx_template: Docx,
}

impl Document {
    pub fn new(name: &str, content: Block) -> Self {
        Self {
            name: name.to_owned(),
            content,

            docx_template: Docx::new(),
        }
    }

    pub fn with_docx_template(self, docx_template: Docx) -> Self {
        Self {
            docx_template,
            ..self
        }    
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn docx(&self) -> Docx {
        self.docx_template.clone()
    }

    pub fn content(&self) -> Block {
        self.content.clone()
    }
}

pub fn print_placeholders(block: &Block) {
    match &block {
        Block::Placeholder(inner, placeholder) => {
            warn!("adding placeholder with description: {:?}", placeholder);
            print_placeholders(&*inner);
        },
        Block::Multiple(multiple) => multiple.iter().for_each(print_placeholders),
        Block::SectionHeader(_) => (),
        Block::SubsectionHeader(_) => (),
        Block::Paragraph(_) => (),
        Block::UnorderedList(_) => (),
        Block::Image(_) => (),
        Block::ReferencesList(_) => (),
        Block::TableOfContents => (),
        Block::AbstractSection(_, _) => (),
        Block::TaskSection => (),
        Block::FrontPage => (),
        Block::TopicCard => (),
        Block::Note(_) => (),
    }
}

pub fn count_pages(docx: Docx, content: &Content, block: &Block) -> Result<u32, PageCountingError> {
    let mut context = Context::new();

    let docx_path = "./output/tmp.docx";
    let pdf_path = "./output/tmp.pdf";

    docx
        .add_text_block(&mut context, content, block.clone())
        .build()
        .pack(File::create(docx_path).unwrap())
        .unwrap();

    if let Err(err) = Command::new("docx2pdf").args([docx_path, pdf_path]).output() {
        if err.kind() == ErrorKind::NotFound {
            return Err(PageCountingError::NoPdfConverterInstalled.into());
        }
    };

    let pdf = lopdf::Document::load(pdf_path).unwrap();
    Ok(pdf.get_pages().len() as u32)
}

pub fn count_images(block: &Block) -> u32 {
    match &block {
        Block::Placeholder(inner, _) => count_images(&*inner),
        Block::Multiple(multiple) => multiple.iter().map(count_images).sum(),
        Block::SectionHeader(_) => 0,
        Block::SubsectionHeader(_) => 0,
        Block::Paragraph(_) => 0,
        Block::UnorderedList(_) => 0,
        Block::Image(_) => 1,
        Block::ReferencesList(_) => 0,
        Block::TableOfContents => 0,
        Block::AbstractSection(_, _) => 0,
        Block::TaskSection => 0,
        Block::FrontPage => 0,
        Block::TopicCard => 0,
        Block::Note(_) => 0,
    }
}