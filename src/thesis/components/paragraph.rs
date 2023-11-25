use {
    docx_rs::{Docx, Paragraph, Run, Tab, LineSpacing, AlignmentType, BreakType},
    crate::thesis::engine::TextSpan,
    super::PlaceholderComponent,
};

pub trait ParagraphComponent {
    fn add_paragraph_component(self, text: TextSpan, tab: bool, line_spacing: i32) -> Self;
    fn add_paragraph_placeholder_component(self, text: TextSpan, desription: impl Into<String>) -> Self;
}

impl ParagraphComponent for Docx {
    fn add_paragraph_component(self, text: TextSpan, tab: bool, line_spacing: i32) -> Self {
        self.add_paragraph(runs_for_text_span(text, Run::new()).into_iter().fold(paragraph(tab, line_spacing), |p, r| p.add_run(r)))
    }

    fn add_paragraph_placeholder_component(self, text: TextSpan, description: impl Into<String>) -> Self {
        self.add_paragraph(paragraph(true, 24 * 15).add_placeholder_component(text.to_plaintext(), description))
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

fn paragraph(tab: bool, line_spacing: i32) -> Paragraph {
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
        .line_spacing(LineSpacing::new().line(line_spacing))
        .align(AlignmentType::Both)
        .add_run(run)
}
