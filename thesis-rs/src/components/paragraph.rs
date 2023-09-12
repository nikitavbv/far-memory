use {
    docx_rs::{Docx, Paragraph, Run, Tab, LineSpacing, AlignmentType},
    super::PlaceholderComponent,
};

pub trait ParagraphComponent {
    fn add_paragraph_component(self, text: impl Into<String>) -> Self;
    fn add_paragraph_placeholder_component(self, text: impl Into<String>, desription: impl Into<String>) -> Self;
}

impl ParagraphComponent for Docx {
    fn add_paragraph_component(self, text: impl Into<String>) -> Self {
        self.add_paragraph(paragraph().add_run(Run::new().add_text(text)))
    }

    fn add_paragraph_placeholder_component(self, text: impl Into<String>, description: impl Into<String>) -> Self {
        self.add_paragraph(paragraph().add_placeholder_component(text, description))
    }
}

fn paragraph() -> Paragraph {
    Paragraph::new()
        .add_tab(Tab::new().pos(710))
        .line_spacing(LineSpacing::new().line(24 * 15))
        .align(AlignmentType::Both)
        .add_run(Run::new().add_tab())
}