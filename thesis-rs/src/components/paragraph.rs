use docx_rs::{Docx, Paragraph, Run, Tab, LineSpacing, AlignmentType};

pub trait ParagraphComponent {
    fn add_paragraph_component(self, text: impl Into<String>) -> Self;
}

impl ParagraphComponent for Docx {
    fn add_paragraph_component(self, text: impl Into<String>) -> Self {
        self.add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text(text))
        )
    }
}