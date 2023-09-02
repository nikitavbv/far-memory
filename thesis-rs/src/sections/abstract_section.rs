use {
    docx_rs::{Docx, Paragraph, Run, BreakType, AlignmentType, Tab, LineSpacing, NumberingId, IndentLevel},
    crate::{
        content::{Content, Language, MultiLanguageString},
        components::PlaceholderComponent,
    },
};

pub trait AbstractSection {
    fn add_abstract_section(self, content: &Content, language: &Language) -> Self;
}

impl AbstractSection for Docx {
    fn add_abstract_section(self, content: &Content, language: &Language) -> Self {
        let text_title = match language {
            &Language::English => "Abstract",
            &Language::Ukrainian => "Реферат",
        };

        let text_explanatory_note_size = match language {
            &Language::English => "Explanatory note size",
            &Language::Ukrainian => "Розмір пояснювальної записки",
        };

        let total_pages = 90;

        let text_pages_and_contains = match language {
            &Language::English => "pages, contains",
            &Language::Ukrainian => "аркушів, містить",
        };

        let total_pictures = 16;

        let text_pictures = match language {
            &Language::English => "illustrations",
            &Language::Ukrainian => "ілюстрацій",
        };

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

        let text_aim_of_study_description = match language {
            &Language::English => "The main target is ...",
            &Language::Ukrainian => "Основною метою є покращити/підвищити швидкість/зменшити використання ...",
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
            .add_paragraph(Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15).before(200))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text(format!("{} – ", text_explanatory_note_size)))
                .add_placeholder_component(total_pages.to_string(), "replace with an actual number of pages")
                .add_text_component(format!(" {} ", text_pages_and_contains))
                .add_placeholder_component(total_pictures.to_string(), "replace with an actual number of pictures")
                .add_text_component(format!(" {}, ", text_pictures))
                .add_placeholder_component(total_tables.to_string(), "replace with an actual number of tables")
                .add_text_component(format!(" {}, ", text_tables))
                .add_placeholder_component(total_applications.to_string(), "replace with an actual number of applications")
                .add_text_component(format!(" {}, ", text_applications))
                .add_placeholder_component(total_references.to_string(), "replace with an actual number of references")
                .add_text_component(format!(" {}.", text_references))
            )
            .add_paragraph(Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().bold().add_text(format!("{}. ", text_topicality)))
                .add_placeholder_component(text_topicality_description, "replace with correct topicality")
            )
            .add_paragraph(Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().bold().add_text(format!("{}. ", text_aim_of_study)))
                .add_placeholder_component(text_aim_of_study_description, "replace with correct description")
            )
            .add_paragraph(Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text(format!("{}: ", text_object_of_research)).add_text(content.research_object.for_language(language)).add_text("."))
            )
            .add_paragraph(Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text(format!("{}: ", MultiLanguageString::new(
                    "Subject of research",
                    "Предмет дослідження"
                ).for_language(language))))
                .add_placeholder_component(content.research_subject.for_language(language), "update research subject")
                .add_text_component(".")
            )
            .add_paragraph(Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
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
            .add_tasks_component(&[
                MultiLanguageString::new("first task", "перше завдання")
            ], language)
            .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
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
    fn add_tasks_component(self, tasks: &[MultiLanguageString], language: &Language) -> Self;
}

impl TasksComponent for Docx {
    fn add_tasks_component(self, tasks: &[MultiLanguageString], language: &Language) -> Self {
        let mut document = self;

        for i in 0..tasks.len() {
            let task = tasks.get(i).unwrap();

            document = document.add_paragraph(Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_tab())
                .add_text_component("– ")
                .add_placeholder_component(task.for_language(language), "replace with correct task list")
                .add_text_component(if i == tasks.len() - 1 { "." } else { ";" })
            );
        }

        document
    }
}