use {
    docx_rs::{Docx, Run, Paragraph, AlignmentType, LineSpacing},
    crate::components::ParagraphComponent,
};

pub trait AbbreviationsListSection {
    fn add_abbreviations_list_section(self) -> Self;
}

impl AbbreviationsListSection for Docx {
    fn add_abbreviations_list_section(self) -> Self {
        self.add_paragraph(Paragraph::new()
            .line_spacing(LineSpacing::new().after(300))
            .style("Heading1")
            .page_break_before(true)
            .align(AlignmentType::Center)
            .add_run(Run::new().add_text("Перелік умовних позначень".to_uppercase()))
        )
        .add_paragraph_placeholder_component("JDBC – прикладний програмний інтерфейс Java, який визначає методи, з допомогою яких програмне забезпечення на Java здійснює доступ до бази даних;", "add some real definitions")
        .add_paragraph_placeholder_component("Cache – проміжний буфер з швидким доступом, що містить інформацію, яка може бути запрошена з найбільшою ймовірністю.", "add some real definitions")
    }
}