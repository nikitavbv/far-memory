use {
    docx_rs::{Docx, Paragraph, Run, BreakType},
    crate::content::{Content, Language},
};

pub trait AbstractSection {
    fn add_abstract_section(self, content: &Content, language: &Language) -> Self;
}

impl AbstractSection for Docx {
    fn add_abstract_section(self, content: &Content, language: &Language) -> Self {
        self
            .add_paragraph(Paragraph::new()
                .add_run(Run::new()
                    .size(2 * 14)
                    .bold()
                    .add_text("Реферат".to_uppercase())
                )
            )
            .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
    }
}