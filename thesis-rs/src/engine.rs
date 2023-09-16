use {
    docx_rs::{Docx, Paragraph, Tab, LineSpacing, Run},
    crate::{
        context::Context,
        components::{SectionHeaderComponent, ParagraphComponent, UnorderedListComponent, ImageComponent},
    },
};

#[derive(Debug)]
pub enum Block {
    SectionHeader(String),
    SubsectionHeader(String),
    Paragraph(String),
    UnorderedList(Vec<String>),
    Image(ImageBlock),
    Placeholder(Box<Block>, String),
}

#[derive(Debug)]
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

pub fn render_blocks_to_docx(mut document: Docx, context: &mut Context, blocks: Vec<Block>) -> Docx {
    let mut section_index = 0;

    for block in blocks {
        document = match block {
            Block::SectionHeader(text) => {
                section_index = context.next_section_index();
                document.add_section_header_component(format!("{}   {}", section_index, text))
            },
            Block::SubsectionHeader(text) => {
                let subsection_index = context.next_subsection_index(section_index);

                document.add_paragraph(
                    Paragraph::new()
                        .add_tab(Tab::new().pos(710))
                        .line_spacing(LineSpacing::new().before(300).line(24 * 15))
                        .style("Heading2")
                        .add_run(Run::new().add_tab().add_text(format!("{}.{}   {}", section_index, subsection_index, text)))
                )
            },
            Block::Paragraph(text) => document.add_paragraph_component(text),
            Block::UnorderedList(list) => document.add_unordered_list_component(context, list),
            Block::Image(image) => document.add_image_component(context, section_index, &image.path(), &image.description()),
            Block::Placeholder(inner, description) => {
                match *inner {
                    Block::Paragraph(text) => document.add_paragraph_placeholder_component(text, description),
                    other => panic!("block type not supported for placeholder: {:?}", other),
                }
            },
        }
    }

    document
}