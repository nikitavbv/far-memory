use {
    docx_rs::{Docx, PageMargin, RunFonts, SectionType},
    crate::thesis::{
        engine::{Block, ParagraphBlock, TextSpan, SectionHeaderBlock, SubsectionHeaderBlock},
        content::classification_code,
        utils::mm_to_twentieth_of_a_point,
    },
};

const FONT_SIZE: usize = 2 * 12;
const INTERVAL: f32 = 1.15;

pub fn conference_abstract() -> Block {
    Block::Multiple(vec![
        paragraph(TextSpan::Multiple(vec![
            format!("UDC {}", classification_code()).into(),
        ])),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Italic(Box::new(TextSpan::Multiple(vec![
                TextSpan::Bold(Box::new(TextSpan::Regular("Volobuiev Nikita Oleksandrovich".to_owned()))),
                TextSpan::Regular(", master's degree student".to_owned()),
                TextSpan::Break,
                TextSpan::Regular("Igor Sikorsky Kyiv Polytechnic Institute, Ukraine".to_owned()),
            ]))),
            TextSpan::Break,
            TextSpan::Multiple(vec![
                TextSpan::Italic(Box::new(TextSpan::Multiple(vec![
                    TextSpan::Bold(Box::new(TextSpan::Regular("Supervisor: Pavlov Oleksandr Anatoliyovych".to_owned()))),
                    TextSpan::Regular(", Doctor of Engineering Sciences,".to_owned()), // TODO: continue here
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
        end_section(1),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Основна частина.".into())),
            " Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини.  Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини. Текст основної частини.".into(),
        ])),
        end_section(2),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Висновки.".into())),
            " Текст висновків. Текст висновків. Текст висновків. Текст висновків. Текст висновків. Текст висновків. Текст висновків. Текст висновків. Текст висновків. Текст висновків. Текст висновків. Текст висновків.".into(),
        ])),
        Block::SubsectionHeader(
            SubsectionHeaderBlock::without_numbering("Список інформаційних джерел".to_owned())
                .without_tab()
                .center()
                .bold()
                .with_line_spacing(FONT_SIZE, INTERVAL)
        ),
        Block::OrderedList(vec![
            "Виконання основних арифметичних дій з комплексними числами, які представлено в інтервальній гіперболічній формі / С. В. Гадецька [та ін.] // Сучасні інформаційні системи = Advanced Information Systems. – 2022. – Т. 6, № 1. – С. 104-113.".to_owned(),
        ]),
        end_section(1)
    ])
}

fn end_section(columns: usize) -> Block {
    Block::Paragraph(ParagraphBlock::new(TextSpan::Multiple(vec![])).with_tab(false).with_columns(columns))
}

fn paragraph(text: impl Into<TextSpan>) -> Block {
    Block::Paragraph(ParagraphBlock::new(text.into()).with_tab(false).with_line_spacing(FONT_SIZE, INTERVAL).with_after_spacing(300))
}

pub fn conference_abstract_docx_template() -> Docx {
    // requirements: https://docs.google.com/document/d/1CoIPOtUko0ZpV3JgNn9JhV-l_kZDbKO8v66zrIS9dzg/edit
    let mut docx = Docx::new()
        .page_margin(
            PageMargin::new()
                .top(mm_to_twentieth_of_a_point(15.0))
                .bottom(mm_to_twentieth_of_a_point(15.0))
                .left(mm_to_twentieth_of_a_point(20.0))
                .right(mm_to_twentieth_of_a_point(20.0))
        )
        .default_fonts(RunFonts::new().cs("Times New Roman"))
        .default_size(FONT_SIZE)
        .default_tab_stop(0);

    docx.document.section_property.section_type = Some(SectionType::Continuous);

    docx
}
