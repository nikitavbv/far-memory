use {
    docx_rs::{
        Docx,
        Paragraph,
        Run,
        AlignmentType,
        Tab,
        LineSpacing,
        NumberingId,
        IndentLevel,
        Numbering,
        AbstractNumbering,
        Level,
        Start,
        NumberFormat,
        LevelText,
        LevelJc,
        SpecialIndentType,
    },
    crate::thesis::{
        content::{Content, Language, MultiLanguageString, AbstractContent, MultiLanguageNumeralString, EnglishNumeralString, UkrainianNumeralString},
        context::Context,
        components::{PlaceholderComponent, PageBreakComponent},
    },
};

pub trait AbstractSection {
    fn add_abstract_section(self, context: &mut Context, content: &Content, abstract_content: &AbstractContent, language: &Language) -> Self;
}

impl AbstractSection for Docx {
    fn add_abstract_section(self, context: &mut Context, content: &Content, abstract_content: &AbstractContent, language: &Language) -> Self {
        let text_title = match language {
            &Language::English => "Abstract",
            &Language::Ukrainian => "Реферат",
        };

        let text_explanatory_note_size = match language {
            &Language::English => "Explanatory note size",
            &Language::Ukrainian => "Розмір пояснювальної записки",
        };

        let text_pages_and_contains = match language {
            &Language::English => "pages, contains",
            &Language::Ukrainian => "аркушів, містить",
        };

        let text_pictures = MultiLanguageNumeralString::new(
            EnglishNumeralString::new("illustration".to_owned()),
            UkrainianNumeralString::new("ілюстрація".to_owned(), "ілюстації".to_owned(), "ілюстрацій".to_owned()),
        );

        let text_tables = MultiLanguageNumeralString::new(
            EnglishNumeralString::new("table".to_owned()),
            UkrainianNumeralString::new("табиця".to_owned(), "таблиці".to_owned(), "таблиць".to_owned()),
        );

        let text_applications = MultiLanguageNumeralString::new(
            EnglishNumeralString::new("application".to_owned()),
            UkrainianNumeralString::new("додаток".to_owned(), "додатки".to_owned(), "додатків".to_owned()),
        );

        let text_references = MultiLanguageNumeralString::new(
            EnglishNumeralString::new("reference".to_owned()),
            UkrainianNumeralString::new(
                "посилання на джерела".to_owned(),
                "посилання на джерела".to_owned(),
                "посилань на джерела".to_owned(),
            ),
        );

        let text_topicality = match language {
            &Language::English => "Topicality",
            &Language::Ukrainian => "Актуальність теми",
        };

        let text_topicality_description = match language {
            &Language::English => "Increasing resource utilization efficiency (random access memory in particular) is an important problem which arises in modern data \
centers. Memory utilization is uneven between nodes in a computing cluster even with modern schedulers and virtualization. Far memory allows using random-access memory \
more efficiently and evenly, while also allowing to access more memory than available on a compute node. At the same time, existing implementations and methods of \
providing far memory have a limited application scope and low efficiency, which are defined by the specifics of the task. Because of that, increasing efficiency by \
modifying existing methods of providing far memory is relevant.",
            &Language::Ukrainian => "У сучасних центрах обробки даних актуальним є збільшення ефективності використання ресурсів, зокрема оперативної памʼяті серверів. \
Навіть при наяві сучасних планувальників задач та віртуалізації, використання памʼяті є неріномірним між вузлами у обчислювальному кластері. Застосування \
віддаленої памʼяті дозволяє використовувати оперативну памʼять більш оптимально, знижуючи нерівномірність використання цього ресурсу, а також мати доступ до більшого \
обʼєму памʼяті ніж є доступним на одному вузлі. При цьому, існуючі реалізації та методи надання віддаленої памʼяті мають обмежену область застосування та невисоку ефективність, \
зумовлену особливостями задачі. Через це, підвищення ефективності за рахунок модифікації існуючих методів надання віддаленої памʼяті є актуальним.",
        };

        let text_aim_of_study = match language {
            &Language::English => "The aim of the study",
            &Language::Ukrainian => "Мета дослідження",
        };

        let text_object_of_research = match language {
            &Language::English => "Object of research",
            &Language::Ukrainian => "Об’єкт дослідження",
        };

        self
            .add_paragraph(Paragraph::new()
                .align(AlignmentType::Center)
                .page_break_before(true)
                .add_run(Run::new()
                    .size(2 * 14)
                    .bold()
                    .add_text(text_title.to_uppercase())
                )
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().add_text(format!("{} – {}", text_explanatory_note_size, abstract_content.total_pages)))
                .add_text_component(format!(" {} {}", text_pages_and_contains, abstract_content.total_images))
                .add_text_component(format!(" {}, ", text_pictures.for_language_and_value(language, abstract_content.total_images)))
                .add_text_component(format!("{} {}, ", abstract_content.total_tables, text_tables.for_language_and_value(language, abstract_content.total_tables)))
                .add_text_component(format!("{} {}, ", abstract_content.total_applications, text_applications.for_language_and_value(language, abstract_content.total_applications)))
                .add_text_component(format!("{} {}.", abstract_content.total_references, text_references.for_language_and_value(language, abstract_content.total_references)))
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(format!("{}. ", text_topicality)))
                .add_text_component(text_topicality_description)
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(format!("{}. ", text_aim_of_study)))
                .add_run(Run::new().add_text(content.aim_of_study.for_language(language)))
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(format!("{}: ", text_object_of_research)))
                .add_run(Run::new().add_text(format!("{}.", content.research_object.for_language(language))))
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(format!("{}: ", MultiLanguageString::new(
                    "Subject of research",
                    "Предмет дослідження"
                ).for_language(language))))
                .add_text_component(content.research_subject.for_language(language))
                .add_text_component(".")
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().add_text(MultiLanguageString::new(
                    "To achieve this goal, the ",
                    "Для реалізації поставленої мети "
                ).for_language(language)))
                .add_run(Run::new()
                    .bold()
                    .add_text(MultiLanguageString::new(
                        "following tasks",
                        "сформульовані наступні завдання"
                    ).for_language(language))
                )
                .add_text_component(MultiLanguageString::new(
                    " were formulated",
                    ""
                ).for_language(language))
                .add_text_component(":")
            )
            .add_tasks_component(context, &content.tasks, language)
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(MultiLanguageString::new("The scientific novelty", "Наукова новизна:").for_language(language)))
                .add_text_component(" ")
                .add_text_component(content.scientific_novelty.for_language(language))
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(MultiLanguageString::new("The practical value", "Практичне значення").for_language(language)))
                .add_text_component(" ")
                .add_text_component(MultiLanguageString::new(
                    "of the obtained results lies in the fact that the developed software for providing far memory is easy to deploy and does not require significant \
changes to the software during integration. This software can be used to enhance the efficiency of resource utilization in datacenter for software which operating parameters \
allow using such memory class as far memory.",
                    "отриманих результатів полягає в тому, що розроблене програмне забезпечення для надання віддаленої памʼяті є простим для розгортання, не \
вимагає значних змін у програмне забезпечення при інтеграції. Дане програмне забезпечення може бути використане для підвищення ефективності використання ресурсів центру \
обробки даних у програмному забезпченні параметри роботи якого дозволяють використання такого класу памʼяті як віддалена памʼять."
                ).for_language(language))
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(MultiLanguageString::new("Relationship with working with scientific programs, plans, topics.", "Зв’язок з науковими програмами, планами, темами.").for_language(language)))
                .add_text_component(" ")
                .add_text_component(MultiLanguageString::new(
                    "Work was performed at the Department of Informatics and Software Engineering of the National Technical University of Ukraine «Kyiv Polytechnic Institute. Igor Sikorsky»",
                    "Робота виконувалась на кафедрі інформатики та програмної інженерії Національного технічного університету України \"Київський політехнічний інститут імені Ігоря Сікорського\""
                ).for_language(language))
                .add_text_component(".")
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(MultiLanguageString::new("Approbation", "Апробація").for_language(language)).add_text("."))
                .add_text_component(" ")
                .add_text_component(MultiLanguageString::new(
                    "The scientific provisions of the dissertation were tested at the",
                    "Наукові положення дисертації пройшли апробацію на"
                ).for_language(language))
                .add_text_component(" ")
                .add_text_component(MultiLanguageString::new(
                    "V International Scientific and Practical Conference for Young Scientists and Students \"Software Engineering and Advanced Information Technologies SoftTech-2023\"",
                    "V Міжнародній науково-практичній конференції молодих вчених та студентів «Інженерія програмного забезпечення і передові інформаційні технології SoftTech-2023»"
                ).for_language(language))
                .add_text_component(".")
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(MultiLanguageString::new("Publications", "Публікації").for_language(language)).add_text("."))
                .add_text_component(" ")
                .add_text_component(MultiLanguageString::new(
                    "The scientific provisions of the dissertation published in",
                    "Наукові положення дисертації опубліковані в"
                ).for_language(language))
                .add_text_component(":")
            )
            .add_publications_component(context, &[
                MultiLanguageString::new(
                    "Methods and software for providing software-defined far memory in distributed systems/ N.O. Volobuev, O.A. Pavlov, M.M. Holovchenko // Proceedings of the V International Scientific and Practical Conference for Young Scientists and Students \"Software Engineering and Advanced Information Technologies SoftTech-2023\" - Kyiv: National Technical University of Ukraine «Igor Sikorsky Kyiv Polytechnic Institute», December 19-21, 2023.",
                    "Methods and software for providing software-defined far memory in distributed systems/ Н.О. Волобуєв, О.А. Павлов, М.М. Головченко // Матеріали V Міжнародної науково-практичної конференції молодих вчених та студентів «Інженерія програмного забезпечення і передові інформаційні технології SoftTech-2023» – м. Київ: НТУУ «КПІ ім. Ігоря Сікорського», 12-21 грудня 2023 р."
                )
            ], language)
    }
}

trait TextComponent {
    fn add_text_component(self, text: impl Into<String>) -> Self;
}

impl TextComponent for Paragraph {
    fn add_text_component(self, text: impl Into<String>) -> Self {
        self.add_run(Run::new().add_text(text))
    }
}

trait TasksComponent {
    fn add_tasks_component(self, context: &mut Context, tasks: &[MultiLanguageString], language: &Language) -> Self;
}

impl TasksComponent for Docx {
    fn add_tasks_component(self, context: &mut Context, tasks: &[MultiLanguageString], language: &Language) -> Self {
        let tasks_numbering = context.next_numbering_id();

        let mut document = self
            .add_abstract_numbering(
                AbstractNumbering::new(tasks_numbering)
                    .add_level(Level::new(
                        0,
                        Start::new(0),
                        NumberFormat::new("bullet"),
                        LevelText::new("– "),
                        LevelJc::new("left")
                    ).indent(None, Some(SpecialIndentType::FirstLine(725)), None, None))
            )
            .add_numbering(Numbering::new(tasks_numbering, tasks_numbering));

        for i in 0..tasks.len() {
            let task = tasks.get(i).unwrap();

            document = document.add_paragraph_with_abstract_style_component(Paragraph::new()
                .numbering(NumberingId::new(tasks_numbering), IndentLevel::new(0))
                .add_text_component(task.for_language(language))
                .add_text_component(if i == tasks.len() - 1 { "." } else { ";" })
            );
        }

        document
    }
}

trait PublicationsComponent {
    fn add_publications_component(self, context: &mut Context, publications: &[MultiLanguageString], language: &Language) -> Self;
}

impl PublicationsComponent for Docx {
    fn add_publications_component(self, context: &mut Context, publications: &[MultiLanguageString], language: &Language) -> Self {
        let numbering = context.next_numbering_id();

        let document = self
            .add_abstract_numbering(
                AbstractNumbering::new(numbering)
                    .add_level(Level::new(
                        0,
                        Start::new(1),
                        NumberFormat::new("decimal"),
                        LevelText::new("%1) "),
                        LevelJc::new("left")
                    )
                    .indent(None, Some(SpecialIndentType::FirstLine(725)), None, None))
            )
            .add_numbering(Numbering::new(numbering, numbering));

        publications.into_iter()
            .fold(document, |document, publication|
                document.add_paragraph_with_abstract_style_component(Paragraph::new()
                    .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                    .add_text_component(publication.for_language(language))
                )
            )
    }
}

trait ParagraphWithAbstractStyleComponent {
    fn add_paragraph_with_abstract_style_component(self, paragraph: Paragraph) -> Self;
}

impl ParagraphWithAbstractStyleComponent for Docx {
    fn add_paragraph_with_abstract_style_component(self, paragraph: Paragraph) -> Self {
        self.add_paragraph(paragraph.add_tab(Tab::new().pos(710)).line_spacing(LineSpacing::new().line(24 * 15)).align(AlignmentType::Both))
    }
}
