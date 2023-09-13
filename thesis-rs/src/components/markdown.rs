use {
    docx_rs::Docx,
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
        
        for block in markdown::tokenize(md.into().as_str()) {
            document = match block {
                Block::Header(spans, size) => {
                    if size != 1 {
                        panic!("only headers with size 1 are currently supported");
                    }

                    let header_text = spans.into_iter()
                        .fold(format!("{}  ", context.next_section_index()), |prev, span| format!("{}{}", prev, match span {
                            Span::Text(text) => text,
                            other => panic!("unexpected span in markdown header: {:?}", other),
                        }))
                        .to_uppercase();

                    document.add_section_header_component(header_text)
                },
                other => panic!("unexpected markdown block: {:?}", other)
            }
        }

        document
    }
}