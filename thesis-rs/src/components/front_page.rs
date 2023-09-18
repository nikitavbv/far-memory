use {
    docx_rs::{Docx, Paragraph, Run, BreakType, AlignmentType, Table, TableRow, TableCell, LineSpacing, WidthType, TableBorders, VAlignType, TableCellMargins, TableAlignmentType},
    crate::{
        components::LineComponent,
        content::Content,
    },
};

pub trait FrontPageSection {
    fn add_front_page_section(self, content: &Content) -> Self;
}

impl FrontPageSection for Docx {
    fn add_front_page_section(self, content: &Content) -> Self {
        self
            .add_paragraph(Paragraph::new()
                .add_run(Run::new()
                    .size(28)
                    .bold()
                    .add_text("Національний технічний університет України".to_uppercase())
                    .add_break(BreakType::TextWrapping)
                    .add_text("«Київський Політехнічний Інститут".to_uppercase())
                    .add_break(BreakType::TextWrapping)
                    .add_text("імені ")
                    .add_text("Ігоря Сікорського»".to_uppercase())
                )
                .align(AlignmentType::Center))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().line(24 * 15).before(100).after(700))
                .add_run(Run::new()
                    .size(28)
                    .add_text("Факультет інформатики та обчислювальної техніки")
                    .add_break(BreakType::TextWrapping)
                    .add_text("Кафедра інформатики та програмної інженерії")
                )
                .align(AlignmentType::Center))
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                            .add_run(Run::new()
                                .size(24)
                                .add_text("«На правах рукопису»")
                                .add_break(BreakType::TextWrapping)
                                .add_text("УДК "))
                            .add_run(Run::new()
                                .underline("single")
                                .add_text("004.75")
                            ))
                        .width(5000, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                            .line_spacing(LineSpacing::new().line(24 * 15))
                            .add_run(Run::new()
                                .size(24)
                                .add_text("«До захисту допущено»")
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
                .line_spacing(LineSpacing::new().line(30 * 10).before(800).after(300))
                .align(AlignmentType::Center)
                .add_run(Run::new()
                    .size(20 * 2)
                    .bold()
                    .add_text("Магістерська дисертація")
                    .add_break(BreakType::TextWrapping))
                .add_run(Run::new()
                    .size(14 * 2)
                    .bold()
                    .add_text("на здобуття ступеня магістра")
                    .add_break(BreakType::TextWrapping)
                    .add_text("за освітньо-професійною програмою «Інженерія програмного забезпечення інформаційних систем»")
                    .add_break(BreakType::TextWrapping)
                    .add_text("зі спеціальності 121 «Інженерія програмного забезпечення»")
                    .add_break(BreakType::TextWrapping)
                    .add_text(format!("на тему: «{}»", content.topic))))
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                            .add_run(Run::new()
                                .size(14 * 2)
                                .add_text("Виконав:")
                                .add_break(BreakType::TextWrapping)
                                .add_text("студент ІІ курсу, групи ІП-22мп")
                                .add_break(BreakType::TextWrapping)
                                .add_text("Волобуєв Нікіта Олександрович")))
                        .width(7000, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                                .add_run(Run::new().add_line_component(800000)))
                        .vertical_align(VAlignType::Bottom)
                ]),
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                            .add_run(Run::new()
                                .size(14 * 2)
                                .add_text("Керівник: ")
                                .add_break(BreakType::TextWrapping)
                                .add_text(&content.mentor_title)
                                .add_break(BreakType::TextWrapping)
                                .add_text(&content.mentor.full_name())))
                        .width(7000, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                                .add_run(Run::new().add_line_component(800000)))
                        .vertical_align(VAlignType::Bottom)
                ]),
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                            .add_run(Run::new()
                                .size(14 * 2)
                                .highlight("yellow")
                                .add_text("Рецензент:")
                                .add_break(BreakType::TextWrapping)
                                .add_text("доцент кафедри ІСТ, к.т.н., доц.,")
                                .add_break(BreakType::TextWrapping)
                                .add_text("Лісовиченко Олег Іванович ")))
                        .width(7000, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                                .add_run(Run::new().add_line_component(800000)))
                        .vertical_align(VAlignType::Bottom)
                ]),
            ]).set_borders(TableBorders::new().clear_all()).margins(TableCellMargins::new().margin_bottom(300, WidthType::Dxa)))
            .add_paragraph(Paragraph::new())
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new()
                        .align(AlignmentType::Left)
                        .add_run(Run::new()
                            .size(14 * 2)
                            .add_text("Засвідчую, що у цій магістерській дисертації")
                            .add_break(BreakType::TextWrapping)
                            .add_text("немає запозичень з праць інших авторів без")
                            .add_break(BreakType::TextWrapping)
                            .add_text("відповідних посилань.")
                            .add_break(BreakType::TextWrapping)
                            .add_text("Студент (-ка) ")
                            .add_line_component(800000)
                        ))
                        .width(6000, WidthType::Dxa),
                ]),
            ]).align(TableAlignmentType::Right).set_borders(TableBorders::new().clear_all()))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .align(AlignmentType::Center)
                .add_run(Run::new().add_text("Київ – 2023 року"))
            )
    }
}