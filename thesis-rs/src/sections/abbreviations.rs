use {
    docx_rs::{Docx, Run, Paragraph, AlignmentType, Table, TableRow, TableCell, LineSpacing},
    crate::components::PlaceholderComponent,
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
            .add_run(Run::new().add_text("Перелік умовних позначень, СИМВОЛІВ, ОДИНИЦЬ, СКОРОЧЕНЬ І ТЕРМІНІВ"))
        )
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new().add_placeholder_component("API", "add some real terms")),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().add_placeholder_component("– Application programming interface, прикладний програмний Інтерфейс", "add some real terms")),
                ]),
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new().add_placeholder_component("Розподілена система", "add some real terms")),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().add_placeholder_component("–Програмне забезпечення, що складається з декількох компонентів, що виконуються на різних фізичних вузлах.", "add some real terms")),
                ])
            ]).clear_all_border())
    }
}