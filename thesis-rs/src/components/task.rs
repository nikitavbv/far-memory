use {
    docx_rs::{
        Docx, 
        Paragraph, 
        Run, 
        BreakType, 
        LineSpacing, 
        AlignmentType, 
        Table, 
        TableRow, 
        TableCell, 
        WidthType, 
        TableBorders, 
        NumberingId, 
        IndentLevel, 
        VAlignType, 
        VMergeType, 
        TableCellMargins,
        AbstractNumbering,
        Level,
        Start,
        NumberFormat,
        LevelText,
        LevelJc,
    },
    crate::{
        components::{LineComponent, PlaceholderComponent},
        content::{Content, Language},
        context::Context,
    },
};

pub trait TaskSection {
    fn add_task_section(self, context: &mut Context, content: &Content) -> Self;
}

impl TaskSection for Docx {
    fn add_task_section(self, context: &mut Context, content: &Content) -> Self {
        let numbering = context.next_numbering_id();

        self
            .add_abstract_numbering(
                AbstractNumbering::new(numbering)
                    .add_level(Level::new(
                        0,
                        Start::new(1),
                        NumberFormat::new("decimal"),
                        LevelText::new("%1. "),
                        LevelJc::new("start")
                    )
                )
            )
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
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new()
                    .add_text(format!(
                        "Тема дисертації «{}», науковий керівник дисертації {} {}, затверджені наказом по університету від ", 
                        content.topic, 
                        content.mentor.full_name(), 
                        content.mentor_title
                    )))
                .add_placeholder_component("«27» жовтня 2021 р. № 3587-с", "update with correct date and number after it is issued")
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Термін подання студентом дисертації "))
                .add_placeholder_component("«06» грудня 2023 р.", "update with correct date for thesis submit")
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text(format!("Об’єкт дослідження – {}.", content.research_object.for_language(&Language::Ukrainian))))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Предмет дослідження – "))
                .add_run(Run::new().add_text(content.research_subject.for_language(&Language::Ukrainian)))
                .add_run(Run::new().add_text(".")))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Перелік завдань, які потрібно розробити – ").add_text(format!("{}.", content.tasks.join("; "))))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Орієнтовний перелік графічного (ілюстративного) матеріалу – 3 плакати"))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Орієнтовний перелік публікацій – одна публікація"))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_break(BreakType::Page).add_text("Консультанти розділів дисертації"))
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
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Дата видачі завдання «01» вересня 2023 р."))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .align(AlignmentType::Center)
                .add_run(Run::new().add_text("Календарний план"))
            )
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .vertical_align(VAlignType::Center)
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("№ з/п")))
                        .width(500, WidthType::Dxa),
                    TableCell::new()
                        .vertical_align(VAlignType::Center)
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("Назва етапів виконання").add_break(BreakType::TextWrapping).add_text("магістерської дисертації")))
                        .width(5800, WidthType::Dxa),
                    TableCell::new()
                        .vertical_align(VAlignType::Center)
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("Термін виконання")))
                        .width(2000, WidthType::Dxa),
                    TableCell::new()
                        .vertical_align(VAlignType::Center)
                        .add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("Примітка")))
                        .width(1200, WidthType::Dxa),
                ]),
                calendar_plan_empty_row(1),
                calendar_plan_empty_row(2),
                calendar_plan_empty_row(3),
                calendar_plan_empty_row(4),
                calendar_plan_empty_row(5),
                calendar_plan_empty_row(6),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().size(12 * 2).add_text("7"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_placeholder_component("Виконання експериментальних досліджень", "fill this table")),
                    TableCell::new(),
                    TableCell::new(),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().size(12 * 2).add_text("8"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().size(12 * 2).add_text("Оформлення пояснювальної записки"))),
                    TableCell::new(),
                    TableCell::new(),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().size(12 * 2).add_text("9"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().size(12 * 2).add_text("Подання дисертації на попередній захист "))),
                    TableCell::new().add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("22.11.2023"))),
                    TableCell::new(),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().size(12 * 2).add_text("10"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().size(12 * 2).add_text("Подання дисертації на захист"))),
                    TableCell::new().add_paragraph(Paragraph::new().align(AlignmentType::Center).add_run(Run::new().size(12 * 2).add_text("06.12.2023"))),
                    TableCell::new(),
                ]),
            ]).margins(TableCellMargins::new().margin(0, 80, 0, 80)))
            .add_paragraph(Paragraph::new())
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().width(400, WidthType::Dxa),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Студент"))).width(6300, WidthType::Dxa),
                    TableCell::new().add_paragraph(Paragraph::new()
                        .align(AlignmentType::Right)
                        .add_run(Run::new().add_text(format!("Нікіта {}", "Волобуєв".to_uppercase()))
                    )),
                ]),
                TableRow::new(vec![
                    TableCell::new(),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Науковий керівник"))),
                    TableCell::new().add_paragraph(Paragraph::new()
                        .align(AlignmentType::Right)
                        .add_run(Run::new().add_text(format!("{} {}", content.mentor.first_name, content.mentor.last_name.to_uppercase()))
                    )),
                ]),
            ]).clear_all_border().margins(TableCellMargins::new().margin_bottom(400, WidthType::Dxa)))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
    }
}

fn calendar_plan_empty_row(index: u32) -> TableRow {
    TableRow::new(vec![
        TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().size(12 * 2).add_text(index.to_string()))),
        TableCell::new(),
        TableCell::new(),
        TableCell::new(),
    ])
}