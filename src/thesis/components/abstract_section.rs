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
            UkrainianNumeralString::new("ілюстрація".to_owned(), "ілюстації".to_owned()),
        );

        let total_tables = 26;

        let text_tables = match language {
            &Language::English => "tables",
            &Language::Ukrainian => "таблиць",
        };

        let total_applications = 6;

        let text_applications = match language {
            &Language::English => "applications",
            &Language::Ukrainian => "додатків",
        };

        let total_references = 35;

        let text_references = match language {
            &Language::English => "references",
            &Language::Ukrainian => "посилань на джерела",
        };

        let text_topicality = match language {
            &Language::English => "Topicality",
            &Language::Ukrainian => "Актуальність теми",
        };

        let text_topicality_description = match language {
            &Language::English => "Examines the problem of ...",
            &Language::Ukrainian => "У роботі розглянуто проблему в такій-то області з таким-то об’єктом, показано основні особливості існуючих рішень проблеми, їх переваги та недоліки. Виявлено потребу в удосконаленні/розробці того-то."
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
                .add_placeholder_component(total_tables.to_string(), "replace with an actual number of tables")
                .add_text_component(format!(" {}, ", text_tables))
                .add_placeholder_component(total_applications.to_string(), "replace with an actual number of applications")
                .add_text_component(format!(" {}, ", text_applications))
                .add_placeholder_component(total_references.to_string(), "replace with an actual number of references")
                .add_text_component(format!(" {}.", text_references))
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(format!("{}. ", text_topicality)))
                .add_placeholder_component(text_topicality_description, "replace with correct topicality")
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(format!("{}. ", text_aim_of_study)))
                .add_run(Run::new().add_text(content.aim_of_study.for_language(language)))
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().add_text(format!("{}: ", text_object_of_research)).add_text(content.research_object.for_language(language)).add_text("."))
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().add_text(format!("{}: ", MultiLanguageString::new(
                    "Subject of research",
                    "Предмет дослідження"
                ).for_language(language))))
                .add_placeholder_component(content.research_subject.for_language(language), "update research subject")
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
            .add_tasks_component(context, &[
                MultiLanguageString::new("first task", "перше завдання"),
                MultiLanguageString::new("second task", "друге завдання"),
                MultiLanguageString::new("third task", "третє завдання"),
            ], language)
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(MultiLanguageString::new("The scientific novelty", "Наукова новизна").for_language(language)))
                .add_text_component(" ")
                .add_placeholder_component(MultiLanguageString::new(
                    "of the results of the master's dissertation is ...",
                    "результатів магістерської дисертації полягає в тому, що запропоновано архітектурне рішення для побудови програмного забезпечення для створення торгового портфелю, яке на відміну від інших надає користувачеві очікуваний результат при мінімальних затратах часу та кількості необхідних дій для початку роботи. Результат досягнутий шляхом розробки модернізованого алгоритму оптимізації"
                ).for_language(language), "update scientific novelty to a real one")
            )
            .add_paragraph_with_abstract_style_component(Paragraph::new()
                .add_run(Run::new().add_tab().bold().add_text(MultiLanguageString::new("The practical value", "Практичне значення").for_language(language)))
                .add_text_component(" ")
                .add_placeholder_component(MultiLanguageString::new(
                    "of the obtained results is ...",
                    "отриманих результатів полягає в тому, що реалізовані методи поєднані в межах одного застосунку і максимально прості у використанні для користувача. Також реалізовано АРІ-інтерфейс, за допомогою якого результати роботи алгоритмів можуть з легкістю отримувати і застосовувати сторонні сервіси. Дана система може бути використана там-то."
                ).for_language(language), "update practical value to a real one")
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
                .add_placeholder_component(MultiLanguageString::new(
                    "Fifth All-Ukrainian Scientific and Practical Conference of Young Scientists and Students \"Information Systems and Management Technologies\" (ISTU- 2020) - Kyiv",
                    "V всеукраїнській науково-практичній конференції молодих вчених та студентів «Інформаційні системи та технології управління» (ІСТУ-2020) – м. Київ"
                ).for_language(language), "update practical value to a real one")
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
                    "Yasenova A.V. The application of clustering methods on the foreign exchange market / A.V. Yasenova, O.A. Khalus // Proceedings of the Fifth All-Ukrainian Scientific and Practical Conference of Young Scientists and Students \"Information Systems and Management Technologies\" (ISTU- 2020) - Kyiv: NTUU “KPI them. Igor Sikorsky”, November 26-27, 2020.",
                    "Ясенова А.В. Застосування алгоритмів кластеризації на ринку іноземних валют/ А.В.Ясенова, О.А. Халус // Матеріали V всеукраїнської науковопрактичної конференції молодих вчених та студентів «Інформаційні системи та технології управління» (ІСТУ-2020) – м. Київ: НТУУ «КПІ ім. Ігоря Сікорського», 26-27 листопада 2020 р."
                ),
                MultiLanguageString::new(
                    "Yasenova A.V. Review of clustering algorithms // Proceedings of the scientificpractical conference of young scientists and students \"Information Technology\" - Kyiv: NAU, September 6-7, 2020",
                    "Ясенова А.В. Огляд алгоритмів кластеризації // Матеріали науковопрактичної конференції молодих вчених та студентів «Інформаційні технології – м. Київ: НАУ, 6-7 вересня 2020 р."
                )
            ], language)
            .add_keywords_component(&content.keywords, language)
            .add_page_break_component()
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
                    ).indent(Some(1100 + 300), Some(SpecialIndentType::Hanging(300)), None, None))
            )
            .add_numbering(Numbering::new(tasks_numbering, tasks_numbering));

        for i in 0..tasks.len() {
            let task = tasks.get(i).unwrap();

            document = document.add_paragraph_with_abstract_style_component(Paragraph::new()
                .numbering(NumberingId::new(tasks_numbering), IndentLevel::new(0))
                .add_placeholder_component(task.for_language(language), "replace with correct task list")
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
                    .indent(Some(700), Some(SpecialIndentType::Hanging(300)), None, None))
            )
            .add_numbering(Numbering::new(numbering, numbering));

        publications.into_iter()
            .fold(document, |document, publication| 
                document.add_paragraph_with_abstract_style_component(Paragraph::new()
                    .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                    .add_placeholder_component(publication.for_language(language), "replace with correct publication list")
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

trait KeywordsComponent {
    fn add_keywords_component(self, keywords: &[MultiLanguageString], language: &Language) -> Self;
}

impl KeywordsComponent for Docx {
    fn add_keywords_component(self, keywords: &[MultiLanguageString], language: &Language) -> Self {
        let mut paragraph = Paragraph::new()
            .add_run(Run::new().add_tab().bold().add_text(MultiLanguageString::new("Keywords", "Ключові слова").for_language(language)).add_text(":"))
            .add_run(Run::new().add_text(" "));

        for i in 0..keywords.len() {
            let keyword = keywords.get(i).unwrap();

            paragraph = paragraph.add_placeholder_component(keyword.for_language(language).to_uppercase(), "replace with correct keyword");

            if i < keywords.len() - 1 {
                paragraph = paragraph.add_text_component(", ");
            }
        }
    

        self.add_paragraph_with_abstract_style_component(paragraph.add_text_component("."))
    }
}