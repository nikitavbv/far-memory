use docx_rs::{Docx, Paragraph, Run, PageMargin, Font, FontPitchType, RunFonts, AlignmentType, BreakType, LineSpacing, Table, TableRow, TableCell, TableBorders};

fn main() {
    println!("Hello, world!");

    let file = std::fs::File::create("./thesis.docx").unwrap();

    Docx::new()
        .page_margin(
            PageMargin::new()
                .left(mm_to_twentieth_of_a_point(30.0))
                .top(mm_to_twentieth_of_a_point(20.0))
                .bottom(mm_to_twentieth_of_a_point(20.0))
                .right(mm_to_twentieth_of_a_point(10.0))
        )
        .default_fonts(RunFonts::new().cs("Times New Roman"))
        .default_size(28) // 14
        .add_paragraph(Paragraph::new()
            .add_run(Run::new()
                .size(24)
                .bold()
                .add_text("Національний технічний університет україни".to_uppercase())
                .add_break(BreakType::TextWrapping)
                .add_text("«Київскьий Політехнічний Інститут".to_uppercase())
                .add_break(BreakType::TextWrapping)
                .add_text("імені ")
                .add_text("Ігоря Сікорського»".to_uppercase())
            )
            .align(AlignmentType::Center))
        .add_paragraph(Paragraph::new()
            .line_spacing(LineSpacing::new().line(24 * 15).before(100))
            .add_run(Run::new()
                .size(24)
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
                            .add_text("004.043")
                        )), // TODO: check which code exactly should I use
                TableCell::new()
                    .add_paragraph(Paragraph::new()
                        .add_run(Run::new()
                            .size(24)
                            .add_text("«До захисту допущено»")))
            ])
        ]).set_borders(TableBorders::new().clear_all()))
        .build()
        .pack(file)
        .unwrap();
}

fn mm_to_twentieth_of_a_point(mm: f32) -> i32 {
    (mm * 56.6929133858).round() as i32
}