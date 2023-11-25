use {
    docx_rs::{Docx, PageMargin, RunFonts},
    crate::thesis::{
        engine::{Block, ParagraphBlock, TextSpan, SectionHeaderBlock},
        utils::mm_to_twentieth_of_a_point,
    },
};

const FONT_SIZE: usize = 2 * 12;

pub fn conference_abstract() -> Block {
    Block::Multiple(vec![
        paragraph(TextSpan::Multiple(vec![
            "УДК 004.414!!!".into(),
        ])),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Italic(Box::new(TextSpan::Multiple(vec![
                TextSpan::Bold(Box::new(TextSpan::Regular("Грищенко Сергій Володимирович".to_owned()))),
                TextSpan::Regular(", здобувач вищої освіти".to_owned()),
                TextSpan::Break,
                TextSpan::Regular("КПІ ім. Ігоря Сікорського, Україна".to_owned()),
            ]))),
            TextSpan::Break,
            TextSpan::Multiple(vec![
                TextSpan::Italic(Box::new(TextSpan::Multiple(vec![
                    TextSpan::Bold(Box::new(TextSpan::Regular("Науковий керівник: Іваненко Олексій Петрович".to_owned()))),
                    TextSpan::Regular(", доктор технічних наук,".to_owned()),
                    TextSpan::Break,
                    TextSpan::Regular("професор, професор кафедри інформатики та програмної інженерії".to_owned()),
                    TextSpan::Break,
                    TextSpan::Regular("КПІ ім. Ігоря Сікорського, Україна".to_owned()),
                ]))),
            ]),
        ])),
        Block::SectionHeader(
            SectionHeaderBlock::without_numbering("РОЗРОБКА ІНФОРМАЦІЙНОЇ СИСТЕМИ УПРАВЛІННЯ ЗАКЛАДОМ ВИЩОЇ ОСВІТИ".to_owned())
                .do_not_include_in_table_of_contents()
                .without_page_break_before()
        ),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new(TextSpan::Regular("Анотація.".to_owned()))),
            " Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації. Текст анотації.".into(),
            TextSpan::Break,
            TextSpan::Bold(Box::new("КЛЮЧОВІ СЛОВА:".into())),
            " мінімум 3 слова.".into(),
        ])),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Abstract.".into())),
            " Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text. Abstract text.".into(),
            TextSpan::Break,
            TextSpan::Bold(Box::new("KEY WORDS:".into())), // spelling matches the template
            " at least 3 words.".into(),
        ])),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Вступ.".into())),
            " Текст вступу. Текст вступу. Текст вступу. Текст вступу. Текст вступу. Текст вступу. Текст вступу. Текст вступу.".into(),
        ])),
    ])
}

fn paragraph(text: impl Into<TextSpan>) -> Block {
    Block::Paragraph(ParagraphBlock::new(text.into()).with_tab(false).with_line_spacing(FONT_SIZE, 1.15).with_after_spacing(300))
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
        .default_size(FONT_SIZE)
        .default_tab_stop(0)
}
