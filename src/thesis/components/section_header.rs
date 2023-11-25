use {
    docx_rs::{Docx, Paragraph, LineSpacing, AlignmentType, Run},
    crate::thesis::components::PlaceholderComponent,
};

pub trait SectionHeaderComponent {
    fn add_section_header_component(self, text: impl Into<String>, include_in_table_of_contents: bool, page_break_before: bool) -> Self;
    fn add_section_header_placeholder_component(self, text: impl Into<String>, description: impl Into<String>, include_in_table_of_contents: bool, page_break_before: bool) -> Self;
}

impl SectionHeaderComponent for Docx {
    fn add_section_header_component(self, text: impl Into<String>, include_in_table_of_contents: bool, page_break_before: bool) -> Self {
        self.add_paragraph(paragraph_for_header(include_in_table_of_contents, page_break_before).add_run(Run::new().bold().add_text(text.into().to_uppercase())))
    }

    fn add_section_header_placeholder_component(self, text: impl Into<String>, description: impl Into<String>, include_in_table_of_contents: bool, page_break_before: bool) -> Self {
        self.add_paragraph(paragraph_for_header(include_in_table_of_contents, page_break_before).add_placeholder_component(text.into().to_uppercase(), description))
    }
}

fn paragraph_for_header(include_in_table_of_contents: bool, page_break_before: bool) -> Paragraph {
    let paragraph = Paragraph::new()
        .line_spacing(LineSpacing::new().after(300))
        .page_break_before(page_break_before)
        .align(AlignmentType::Center);

    if include_in_table_of_contents {
        paragraph.style("Heading1")
    } else {
        paragraph
    }
}
