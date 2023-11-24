use {
    docx_rs::{Docx, PageMargin, RunFonts},
    crate::thesis::{
        engine::{Block, ParagraphBlock, TextSpan},
        utils::mm_to_twentieth_of_a_point,
    },
};

pub fn conference_abstract() -> Block {
    Block::Multiple(vec![
        paragraph("UDC {code}"),
        line_break(),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Italic("some text".to_owned()),
        ])),
    ])
}

fn line_break() -> Block {
    paragraph("")
}

fn paragraph(text: impl Into<TextSpan>) -> Block {
    Block::Paragraph(ParagraphBlock::new(text.into()).with_tab(false))
}

pub fn conference_abstract_docx_template() -> Docx {
    // requirements: https://docs.google.com/document/d/1CoIPOtUko0ZpV3JgNn9JhV-l_kZDbKO8v66zrIS9dzg/edit
    Docx::new()
        .page_margin(
            PageMargin::new()
                .top(mm_to_twentieth_of_a_point(15.0))
                .bottom(mm_to_twentieth_of_a_point(15.0))
                .left(mm_to_twentieth_of_a_point(20.0))
                .right(mm_to_twentieth_of_a_point(20.0))
        )
        .default_fonts(RunFonts::new().cs("Times New Roman"))
        .default_size(2 * 12)
        .default_tab_stop(0)
}
