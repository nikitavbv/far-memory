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
        end_section(1),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Introduction.".into())),
            " Modern datacenters rely on various approaches to improving resource efficiency. For instance, CPU oversubscription is frequently used to improve \
CPU compute time utilization. Another resource is persistent storage for which resource disaggregation is applied in modern infrastructure. Instead of storage \
devices being located on individual compute nodes (servers), it is separated into dedicated storage infrastructure which forms a pool of storage shared \
between all compute nodes. In this configuration, access to data stored on drives is provided over the network. This allows to assign as much storage to compute \
nodes as it is needed by the software running on them. This also avoids situation where storage space on individual nodes remains unused because tasks running \
have lower storage requests than what is provided by compute node hardware.".into(),
        ])),
        paragraph_without_after_space(
            "For random access memory (RAM), operators of world's largest datacenters report average utilization of around 60%. Just as with storage, \
some compute nodes in the cluster may be running software that requires less memory than what the hardware provides. Efficiency of task scheduling is unrelated \
to this problem, because compute nodes may be constrained by some other resource (for example, CPU compute time). Following the exact same approach with RAM as \
with persistent storage is problematic due to more strict performance requirements set for this class of memory. Separating RAM into dedicated infrastructure \
that is accessed over the network significantly affects latency and bandwidth numbers for memory access operations. This difference is enough for typical software \
running on compute nodes to noticably degrade in peformance, breaching service level objectives (SLOs) defined for this software."),
        paragraph_without_after_space("One approach to solve this is software-defined far memory. The idea behind this method is that some chunks of data can be \
moved from compute nodes with heavy RAM utilization to nodes where there is a lot of free RAM and access this data over the network when needed in a way that is \
transparent to the software (working with data in far memory should be similar to working with data in regular RAM while requiring little or none changes to software \
source code). This results in higher memory utilization overall while also allowing software to process datasets that are larger in size than RAM available on \
single compute node."),
        paragraph_without_after_space("The goal of far memory is to move as many data as possible from local memory to the memory of remote nodes while also solving challenges that \
come up in this configuration. Far memory implementation should ensure high performance of memory access operations, provide fault tolerance given expanded \
failure domain and integrate into software without significant changes to the codebase and without relying on additional hardware."),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Overview of existing implementations.".into())), // TODO: this should be replaced with something better. For example, "exisisting implementations", "far memory integration into software", etc.
            " There are not many existing implementations of far memory because this topic became interesting for operators of the largest datacenters relatively \
recently. At the time of writing, Carbink is considered a state of the art far memory implementation, while other notable implementations include ...".into(),
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
    paragraph_with_params(text, true)
}

fn paragraph_without_after_space(text: impl Into<TextSpan>) -> Block {
    paragraph_with_params(text, false)
}

fn paragraph_with_params(text: impl Into<TextSpan>, after_spacing: bool) -> Block {
    let block = ParagraphBlock::new(text.into()).with_tab(false).with_line_spacing(FONT_SIZE, INTERVAL);
    let block = if after_spacing {
        block.with_after_spacing(300)
    } else {
        block
    };
    Block::Paragraph(block)
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
