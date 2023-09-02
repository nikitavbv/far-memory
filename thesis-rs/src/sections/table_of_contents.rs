use docx_rs::{Docx, TableOfContents, Paragraph, Run};

pub trait TableOfContentsSection {
    fn add_table_of_contents_component(self) -> Self;
}

impl TableOfContentsSection for Docx {
    fn add_table_of_contents_component(self) -> Self {
        self
            .add_table_of_contents(TableOfContents::new().heading_styles_range(1, 3).alias("Table of contents").auto())
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("hello world")).style("Heading1").page_break_before(true))
    }
}