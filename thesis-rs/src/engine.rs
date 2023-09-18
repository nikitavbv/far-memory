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
        components::{SectionHeaderComponent, ParagraphComponent, UnorderedListComponent, ImageComponent},
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

pub fn render_block_to_docx(document: Docx, context: &mut Context, block: Block) -> Docx {
    render_block_to_docx_with_params(document, context, None, block)
}

fn render_block_to_docx_with_params(document: Docx, context: &mut Context, placeholder: Option<String>, block: Block) -> Docx {
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
        Block::Placeholder(inner, description) => render_block_to_docx_with_params(document, context, Some(description), *inner),
        Block::Multiple(blocks) => blocks.into_iter().fold(document, |doc, block| render_block_to_docx_with_params(doc, context, placeholder.clone(), block)),
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
    fn add_text_block(self, context: &mut Context, block: Block) -> Self;
}

impl TextBlockComponent for Docx {
    fn add_text_block(self, context: &mut Context, block: Block) -> Self {
        render_block_to_docx(self, context, block)
    }
}