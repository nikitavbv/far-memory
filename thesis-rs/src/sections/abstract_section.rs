use {
    docx_rs::{Docx, Paragraph, Run, BreakType, AlignmentType, Tab, LineSpacing},
    crate::{
        content::{Content, Language},
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
                .add_run(Run::new().add_tab().add_text(format!("{} – ", text_explanatory_note_size)))
                .add_placeholder_component(total_pages.to_string(), "replace with an actual number of pages")
                .add_run(Run::new().add_text(format!(" {} ", text_pages_and_contains)))
                .add_placeholder_component(total_pictures.to_string(), "replace with an actual number of pictures")
                .add_run(Run::new().add_text(format!(" {}, ", text_pictures)))
                .add_placeholder_component(total_tables.to_string(), "replace with an actual number of tables")
                .add_run(Run::new().add_text(format!(" {}, ", text_tables)))
                .add_placeholder_component(total_applications.to_string(), "replace with an actual number of applications")
                .add_run(Run::new().add_text(format!(" {}, ", text_applications)))
                .add_placeholder_component(total_references.to_string(), "replace with an actual number of references")
                .add_run(Run::new().add_text(format!(" {}.", text_references)))
            )
            .add_paragraph(Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .add_run(Run::new().add_tab().bold().add_text("Topicality."))
            )
            .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
    }
}