use {
    docx_rs::{Docx, PageMargin, RunFonts},
    crate::thesis::utils::mm_to_twentieth_of_a_point,
};

pub fn topic_card_docx_template() -> Docx {
    Docx::new()
        .page_margin(
            PageMargin::new()
                .left(mm_to_twentieth_of_a_point(10.0))
                .top(mm_to_twentieth_of_a_point(10.0))
                .bottom(mm_to_twentieth_of_a_point(9.6))
                .right(mm_to_twentieth_of_a_point(9.7))   
        )
        .default_fonts(RunFonts::new().cs("Arial").hi_ansi("Arial"))
        .default_size(2 * 11)
        .default_tab_stop(0)
}