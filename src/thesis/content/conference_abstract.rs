use {
    docx_rs::{Docx, PageMargin, RunFonts, SectionType},
    itertools::Itertools,
    crate::thesis::{
        engine::{Block, ParagraphBlock, TextSpan, SectionHeaderBlock, SubsectionHeaderBlock},
        content::{classification_code, keywords, Language},
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
                TextSpan::Regular("National Technical University of Ukraine «Igor Sikorsky Kyiv Polytechnic Institute», Ukraine".to_owned()),
            ]))),
            TextSpan::Break,
            TextSpan::Multiple(vec![
                TextSpan::Italic(Box::new(TextSpan::Multiple(vec![
                    TextSpan::Bold(Box::new(TextSpan::Regular("Supervisor: Pavlov Oleksandr Anatoliyovych".to_owned()))),
                    TextSpan::Regular(", doctor of technical sciences,".to_owned()),
                    TextSpan::Break,
                    TextSpan::Regular("professor, professor of computer science and software engineering department".to_owned()),
                    TextSpan::Break,
                    TextSpan::Regular("National Technical University of Ukraine «Igor Sikorsky Kyiv Polytechnic Institute», Ukraine".to_owned()),
                ]))),
            ]),
        ])),
        Block::SectionHeader(
            SectionHeaderBlock::without_numbering("Methods and software for providing software-defined far memory in distributed systems".to_uppercase())
                .do_not_include_in_table_of_contents()
                .without_page_break_before()
        ),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new(TextSpan::Regular("Анотація.".to_owned()))),
            " У роботі розглянуто проблему надання віддаленої памʼяті у розподілених системах. Розглянуто підходи до інтеграції віддаленої памʼяті у програмне \
забезпечення, забезпечення відмовостійкості та високого рівня швидкодії. Запропоновано удосконалений алгоритм заміщення проміжків, що спирається на статистику \
доступів до проміжків памʼяті для більш ефективного переміщення проміжків між локальною та віддаленою памʼяттю, що дозволяє знизити затримку доступу до даних \
у порівнянні з більш простими підходами заміщення проміжків.".into(),
            TextSpan::Break,
            TextSpan::Bold(Box::new("КЛЮЧОВІ СЛОВА:".into())),
            " ".into(),
            TextSpan::Multiple(keywords().into_iter().map(|v| v.for_language(&Language::Ukrainian).into()).intersperse(", ".into()).collect()),
            ".".into(),
        ])),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Abstract.".into())),
            " This paper discusses the problem of providing far memory in distributed systems. The approaches to integrating far memory into software, ensuring \
fault tolerance and high performance are reviewed. An improved span replacement algorithm is proposed which is based on memory span access statistics and provides \
more efficient swapping between local and remote memory. This results in reduction of data access latency compared to simple span replacement approaches.".into(),
            TextSpan::Break,
            TextSpan::Bold(Box::new("KEYWORDS:".into())),
            " ".into(),
            TextSpan::Multiple(keywords().into_iter().map(|v| v.for_language(&Language::English).into()).intersperse(", ".into()).collect()),
            ".".into(),
        ])),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Introduction.".into())),
            " Modern datacenters rely on various approaches to improving resource utilization.".into(),
        ])),
        end_section(1),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Main part.".into())), // TODO: this should be replaced with something better. For example, "background", "exisisting implementations", "far memory integration into software", etc.
            " Main part text.".into(),
        ])),
        end_section(2),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Conclusion.".into())),
            " Conclusion text.".into(),
        ])),
        Block::SubsectionHeader(
            SubsectionHeaderBlock::without_numbering("References.".to_owned())
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
