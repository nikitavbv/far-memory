use docx_rs::{Docx, Paragraph, Run, BreakType};

pub trait PageBreakComponent {
    fn add_page_break_component(self) -> Self;
}

impl PageBreakComponent for Docx {
    fn add_page_break_component(self) -> Self {
        self.add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::Page)))
    }
}