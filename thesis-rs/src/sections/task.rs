use {
    docx_rs::{Docx, Paragraph, Run, BreakType, LineSpacing, AlignmentType, Table, TableRow, TableCell, WidthType, TableBorders, NumberingId, IndentLevel, VAlignType, VMergeType},
    crate::{
        components::LineComponent,
        content::Content,
    },
};

pub trait TaskSection {
    fn add_task_section(self, content: &Content) -> Self;
}

impl TaskSection for Docx {
    fn add_task_section(self, content: &Content) -> Self {
        self
            .add_paragraph(Paragraph::new()
                .add_run(Run::new()
                    .size(28)
                    .bold()
                    .add_text("Національний технічний університет України")
                    .add_break(BreakType::TextWrapping)
                    .add_text("«Київський політехнічний інститут імені Ігоря Сікорського»")
                )
                .align(AlignmentType::Center))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().line(24 * 15).before(100))
                .add_run(Run::new()
                    .size(28)
                    .add_text("Факультет інформатики та обчислювальної техніки")
                    .add_break(BreakType::TextWrapping)
                    .add_text("Кафедра інформатики та програмної інженерії")
                )
                .align(AlignmentType::Center))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().line(24 * 15).after(400))
                .align(AlignmentType::Both)
                .add_run(Run::new()
                    .size(28)
                    .add_text("Рівень вищої освіти – другий (магістерський)")
                    .add_break(BreakType::TextWrapping)
                    .add_text("Спеціальність – 121 «Інженерія програмного забезпечення»")
                    .add_break(BreakType::TextWrapping)
                    .add_text("Освітньо-професійна програма «Інженерія програмного забезпечення інформаційних систем»")
                )
            )
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().width(5000, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                            .line_spacing(LineSpacing::new().line(24 * 15))
                            .add_run(Run::new()
                                .size(24)
                                .add_text("Затверджую".to_uppercase())
                                .add_break(BreakType::TextWrapping)
                                .add_text("Завідувач кафедри")
                                .add_break(BreakType::TextWrapping)
                                .add_line_component(1000000)
                                .add_text("Едуард ЖАРІКОВ")
                                .add_break(BreakType::TextWrapping)
                                .add_text("«")
                                .add_line_component(180000)
                                .add_text("»")
                                .add_line_component(1400000)
                                .add_text("2023р.")))
                        .width(4000, WidthType::Dxa)
                ])
            ]).set_borders(TableBorders::new().clear_all()))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().line(24 * 10).before(300))
                .align(AlignmentType::Center)
                .add_run(Run::new()
                    .bold()
                    .add_text("Завдання".to_uppercase())
                    .add_break(BreakType::TextWrapping)
                    .add_text("на магістерську дисертацію студенту")
                )
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .align(AlignmentType::Center)
                .add_run(Run::new().bold().add_text("Волобуєву Нікіті Олександровичу"))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new()
                    .add_text(format!(
                        "Тема дисертації «{}», науковий керівник дисертації {} {}, затверджені наказом по університету від ", 
                        content.topic, 
                        content.mentor_name, 
                        content.mentor_title
                    )))
                .add_run(Run::new()
                    .highlight("yellow")
                    .add_text("«27» жовтня 2021 р. № 3587-с"))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Термін подання студентом дисертації "))
                .add_run(Run::new().highlight("yellow").add_text("«06» грудня 2023 р."))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text(format!("Об’єкт дослідження – {}.", content.research_object)))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text(format!("Предмет дослідження – {}.", content.research_subject))))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Перелік завдань, які потрібно розробити – "))
                .add_run(Run::new().highlight("yellow").add_text("аналіз проблеми та існуючих рішень; розробка моделі/методу/алгоритму/програмного забезпечення; дослідження ефективності розробленого методу/алгоритму/програмного забезпечення."))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Орієнтовний перелік графічного (ілюстративного) матеріалу – 3 плакати"))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Орієнтовний перелік публікацій – одна публікація"))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Консультанти розділів дисертації"))
            )
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .vertical_align(VAlignType::Center)
                        .add_paragraph(Paragraph::new()
                            .align(AlignmentType::Center)
                            .add_run(Run::new()
                                .size(12 * 2)
                                .add_text("Розділ")
                            )
                        )
                        .vertical_merge(VMergeType::Restart)
                        .width(3000, WidthType::Dxa),
                    TableCell::new()
                        .vertical_align(VAlignType::Center)
                        .add_paragraph(Paragraph::new()
                            .align(AlignmentType::Center)
                            .add_run(Run::new()
                                .size(12 * 2)
                                .add_text("Прізвище, ініціали та посада консультанта")
                            )
                        )
                        .vertical_merge(VMergeType::Restart),
                    TableCell::new()
                        .vertical_align(VAlignType::Center)
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("Підпис, дата")))
                        .grid_span(2),
                ]),
                TableRow::new(vec![
                    TableCell::new().vertical_merge(VMergeType::Continue),
                    TableCell::new().vertical_merge(VMergeType::Continue),
                    TableCell::new().vertical_align(VAlignType::Center).add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("завдання видав"))),
                    TableCell::new().vertical_align(VAlignType::Center).add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("завдання прийняв"))),
                ]),
                TableRow::new(vec![
                    TableCell::new(),
                    TableCell::new(),
                    TableCell::new(),
                    TableCell::new(),
                ]),
            ]))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(1), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Дата видачі завдання "))
                .add_run(Run::new().highlight("yellow").add_text("«01» вересня 202Х р."))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .align(AlignmentType::Center)
                .add_run(Run::new().add_text("Календарний план"))
            )
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("№ з/п"))),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("Назва етапів виконання магістерської дисертації"))),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("Термін виконання"))),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("Примітка"))),
                ])
            ]))
    }
}