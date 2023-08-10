use docx_rs::{Docx, Paragraph, Run, PageMargin, Font, FontPitchType, RunFonts, AlignmentType};

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
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Національний технічний університет україни".to_uppercase())).align(AlignmentType::Center))
        .build()
        .pack(file)
        .unwrap();
}

fn mm_to_twentieth_of_a_point(mm: f32) -> i32 {
    (mm * 56.6929133858).round() as i32
}