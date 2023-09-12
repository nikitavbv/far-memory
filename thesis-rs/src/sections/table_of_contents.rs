use docx_rs::{Docx, TableOfContents, Paragraph, Run, AlignmentType, TabLeaderType};

pub trait TableOfContentsSection {
    fn add_table_of_contents_section(self) -> Self;
}

impl TableOfContentsSection for Docx {
    fn add_table_of_contents_section(self) -> Self {
        self
            .add_paragraph(Paragraph::new().page_break_before(true).align(AlignmentType::Center).add_run(Run::new().bold().add_text("Зміст".to_uppercase())))
            .add_table_of_contents(TableOfContents::new()
                .heading_styles_range(1, 3)
                .tab_leader_type(Some(TabLeaderType::None))
                .auto()
            )
    }
}