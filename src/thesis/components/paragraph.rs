use {
    docx_rs::{Docx, Paragraph, Run, Tab, LineSpacing, AlignmentType, BreakType, SectionProperty, PageMargin},
    crate::thesis::{engine::TextSpan, utils::mm_to_twentieth_of_a_point},
    super::PlaceholderComponent,
};

pub trait ParagraphComponent {
    fn add_paragraph_component(self, text: TextSpan, tab: bool, line_spacing: i32, after_spacing: Option<u32>, columns: Option<usize>) -> Self;
    fn add_paragraph_placeholder_component(self, text: TextSpan, description: impl Into<String>) -> Self;
}

impl ParagraphComponent for Docx {
    fn add_paragraph_component(self, text: TextSpan, tab: bool, line_spacing: i32, after_spacing: Option<u32>, columns: Option<usize>) -> Self {
        self.add_paragraph(runs_for_text_span(text, Run::new()).into_iter().fold(paragraph(tab, line_spacing, after_spacing, columns), |p, r| p.add_run(r)))
    }

    fn add_paragraph_placeholder_component(self, text: TextSpan, description: impl Into<String>) -> Self {
        self.add_paragraph(paragraph(true, 24 * 15, None, None).add_placeholder_component(text.to_plaintext(), description))
    }
}

fn runs_for_text_span(text: TextSpan, run: Run) -> Vec<Run> {
    match text {
        TextSpan::Bold(inner) => runs_for_text_span(*inner, run.bold()),
        TextSpan::Italic(inner) => runs_for_text_span(*inner, run.italic()),
        TextSpan::Link { .. } => unimplemented!(),
        TextSpan::Regular(text) => vec![run.add_text(text)],
        TextSpan::Multiple(texts) => texts.into_iter().flat_map(|text| runs_for_text_span(text, run.clone()).into_iter()).collect(),
        TextSpan::Break => vec![run.add_break(BreakType::TextWrapping)],
    }
}

fn paragraph(tab: bool, line_spacing: i32, after_spacing: Option<u32>, columns: Option<usize>) -> Paragraph {
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

    let line_spacing = LineSpacing::new().line(line_spacing);
    let line_spacing = if let Some(after_spacing) = after_spacing {
        line_spacing.after(after_spacing)
    } else {
        line_spacing
    };

    let section_property = columns.map(|columns| {
        let mut section = SectionProperty::new();
        section.columns = columns;
        section.space = 720;
        // TODO: do not hardcode page margin
        section.page_margin = PageMargin::new()
            .top(mm_to_twentieth_of_a_point(15.0))
            .bottom(mm_to_twentieth_of_a_point(15.0))
            .left(mm_to_twentieth_of_a_point(20.0))
            .right(mm_to_twentieth_of_a_point(20.0));
        section.section_type = Some(docx_rs::SectionType::Continuous);

        section
    });

    let paragraph = paragraph
        .line_spacing(line_spacing)
        .align(AlignmentType::Both);

    let paragraph = if let Some(section_property) = section_property {
        paragraph.section_property(section_property)
    } else {
        paragraph
    };

    paragraph.add_run(run)
}
