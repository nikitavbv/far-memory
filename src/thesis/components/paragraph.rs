use {
    docx_rs::{Docx, Paragraph, Run, Tab, LineSpacing, AlignmentType},
    crate::thesis::engine::TextSpan,
    super::PlaceholderComponent,
};

pub trait ParagraphComponent {
    fn add_paragraph_component(self, text: TextSpan, tab: bool) -> Self;
    fn add_paragraph_placeholder_component(self, text: TextSpan, desription: impl Into<String>) -> Self;
}

impl ParagraphComponent for Docx {
    fn add_paragraph_component(self, text: TextSpan, tab: bool) -> Self {
        self.add_paragraph(paragraph(tab).add_run(run_for_text_span(text, Run::new())))
    }

    fn add_paragraph_placeholder_component(self, text: TextSpan, description: impl Into<String>) -> Self {
        self.add_paragraph(paragraph(true).add_placeholder_component(text.to_plaintext(), description))
    }
}

fn run_for_text_span(text: TextSpan, run: Run) -> Run {
    match text {
        TextSpan::Bold(text) => run.bold().add_text(text).disable_bold(),
        TextSpan::Italic(text) => run.italic().add_text(text).disable_italic(),
        TextSpan::Link { .. } => unimplemented!(),
        TextSpan::Regular(text) => run.add_text(text),
        TextSpan::Multiple(texts) => texts.into_iter().fold(run, |r, t| run_for_text_span(t, r)),
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
