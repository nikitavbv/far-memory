use {
    docx_rs::{Docx, Paragraph, Run, Tab, LineSpacing, AlignmentType},
    super::PlaceholderComponent,
};

pub trait ParagraphComponent {
    fn add_paragraph_component(self, text: impl Into<String>, tab: bool) -> Self;
    fn add_paragraph_placeholder_component(self, text: impl Into<String>, desription: impl Into<String>) -> Self;
}

impl ParagraphComponent for Docx {
    fn add_paragraph_component(self, text: impl Into<String>, tab: bool) -> Self {
        self.add_paragraph(paragraph(tab).add_run(Run::new().add_text(text)))
    }

    fn add_paragraph_placeholder_component(self, text: impl Into<String>, description: impl Into<String>) -> Self {
        self.add_paragraph(paragraph(true).add_placeholder_component(text, description))
    }
}

fn paragraph(tab: bool) -> Paragraph {
    let paragraph = Paragraph::new();

    let paragraph = if tab {
        paragraph.add_tab(Tab::new().pos(710))
    } else {
        paragraph
    };

    let run = Run::new();
    let run = if tab {
        run.add_tab()
    } else {
        run
    };

    paragraph
        .line_spacing(LineSpacing::new().line(24 * 15))
        .align(AlignmentType::Both)
        .add_run(run)
}
