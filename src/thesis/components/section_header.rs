use {
    docx_rs::{Docx, Paragraph, LineSpacing, AlignmentType, Run},
    crate::thesis::components::PlaceholderComponent,
};

pub trait SectionHeaderComponent {
    fn add_section_header_component(self, text: impl Into<String>) -> Self;
    fn add_section_header_placeholder_component(self, text: impl Into<String>, description: impl Into<String>) -> Self;
}

impl SectionHeaderComponent for Docx {
    fn add_section_header_component(self, text: impl Into<String>) -> Self {
        self.add_paragraph(paragraph_for_header().add_run(Run::new().add_text(text)))
    }

    fn add_section_header_placeholder_component(self, text: impl Into<String>, description: impl Into<String>) -> Self {
        self.add_paragraph(paragraph_for_header().add_placeholder_component(text, description))
    }
}

fn paragraph_for_header() -> Paragraph {
    Paragraph::new()
        .line_spacing(LineSpacing::new().after(300))
        .style("Heading1")
        .page_break_before(true)
        .align(AlignmentType::Center)
}