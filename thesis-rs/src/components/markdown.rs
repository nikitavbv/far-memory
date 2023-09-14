use {
    docx_rs::{Docx, Paragraph, Tab, LineSpacing, Run},
    markdown::{Block, Span},
    crate::{
        components::SectionHeaderComponent,
        context::Context,
    },
};

pub trait MarkdownComponent {
    fn add_markdown_component(self, context: &mut Context, markdown: impl Into<String>) -> Self;
}

impl MarkdownComponent for Docx {
    fn add_markdown_component(self, context: &mut Context, md: impl Into<String>) -> Self {
        let mut document = self;

        let mut header_size_1_index = None;
        
        for block in markdown::tokenize(md.into().as_str()) {
            document = match block {
                Block::Header(spans, size) => {
                    let header_text = spans.into_iter()
                        .fold("".to_owned(), |prev, span| format!("{}{}", prev, match span {
                            Span::Text(text) => text,
                            other => panic!("unexpected span in markdown header: {:?}", other),
                        }))
                        .to_uppercase();

                    match size {
                        1 => {
                            header_size_1_index = Some(context.next_section_index());
                            document.add_section_header_component(format!("{}   {}", header_size_1_index.unwrap(), header_text))
                        },
                        2 => {
                            let section_index = match header_size_1_index {
                                Some(v) => v,
                                None => panic!("cannot insert markdown header with size 2 when no header with size 1 is present"),
                            };
                            let subsection_index = context.next_subsection_index(section_index);

                            document.add_paragraph(
                                Paragraph::new()
                                    .add_tab(Tab::new().pos(710))
                                    .line_spacing(LineSpacing::new().before(300).line(24 * 15))
                                    .style("Heading2")
                                    .add_run(Run::new().add_tab().add_text(format!("{}.{}   {}", section_index, subsection_index, header_text)))
                            )
                        },
                        other => panic!("markdown headers with size {} are not supported yet", other)
                    }
                },
                other => panic!("unexpected markdown block: {:?}", other)
            }
        }

        document
    }
}