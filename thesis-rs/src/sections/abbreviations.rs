use docx_rs::{Docx, Run, Paragraph, AlignmentType};

pub trait AbbreviationsListSection {
    fn add_abbreviations_list_section(self) -> Self;
}

impl AbbreviationsListSection for Docx {
    fn add_abbreviations_list_section(self) -> Self {
        self.add_paragraph(Paragraph::new()
            .style("Heading1")
            .page_break_before(true)
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Перелік скорочень".to_uppercase())))
    }
}