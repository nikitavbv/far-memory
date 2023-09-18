use crate::components::{FrontPageSection, TopicCardDocument};

use {
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
    crate::{
        context::Context,
        content::{Content, Language},
        components::{SectionHeaderComponent, ParagraphComponent, UnorderedListComponent, ImageComponent, AbstractSection, TaskSection},
    },
};

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
    AbstractSection(Language),
    TaskSection,
    FrontPage,
    TopicCard,
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
        Block::AbstractSection(language) => document.add_abstract_section(context, content, &language),
        Block::TaskSection => document.add_task_section(context, content),
        Block::FrontPage => document.add_front_page_section(content),
        Block::TopicCard => document.add_topic_card_document(context, content),
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