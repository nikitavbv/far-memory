use {
    std::process::Command,
    clap::Parser,
    docx_rs::{
        Docx,
        Paragraph, 
        Run, 
        PageMargin, 
        RunFonts, 
        AlignmentType, 
        BreakType, 
        LineSpacing, 
        Table, 
        TableRow, 
        TableCell, 
        TableBorders, 
        WidthType,
        VAlignType,
        TableCellMargins,
        TableAlignmentType,
        NumberingId,
        IndentLevel,
        Numbering,
        AbstractNumbering,
        LevelOverride,
        Level,
        Start,
        NumberFormat,
        LevelText,
        LevelJc,
        SpecialIndentType,
    },
    crate::components::LineComponent,
};

pub mod components;

#[derive(Parser, Debug)]
struct Args {   
    #[arg(short, long)]
    pdf: bool,
}

fn main() {
    let args = Args::parse();

    let path = "./thesis.docx";
    let file = std::fs::File::create(path).unwrap();

    println!("generating thesis to {:?}", path);

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
        .add_abstract_numbering(
            AbstractNumbering::new(1)
                .add_level(Level::new(
                    0,
                    Start::new(1),
                    NumberFormat::new("decimal"),
                    LevelText::new("%1."),
                    LevelJc::new("start")
                )
                .indent(Some(360), Some(SpecialIndentType::Hanging(360)), None, Some(0))
            )
        )
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
                .add_text("на тему: «Методи та програмні засоби надання програмно-визначеної віддаленої памʼяті у розподілених системах»")))
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
                            .add_text("д.т.н., проф., засл.діяч")
                            .add_break(BreakType::TextWrapping)
                            .add_text("Павлов Олександр Анатолійович")))
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
            .add_run(Run::new().add_text("Тема дисертації"))
        )
        .build()
        .pack(file)
        .unwrap();

    if args.pdf {
        println!("converting to pdf");
        Command::new("docx2pdf").args(["./thesis.docx", "./thesis.pdf"]).output().unwrap();
    
        println!("done, opening resulting file");
        Command::new("open").args(["./thesis.pdf"]).output().unwrap();
    }
}

fn mm_to_twentieth_of_a_point(mm: f32) -> i32 {
    (mm * 56.6929133858).round() as i32
}
