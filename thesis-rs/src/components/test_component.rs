use docx_rs::{Docx, Paragraph, Run};

pub trait TestComponent {
    fn add_test_component(self) -> Self;
}

impl TestComponent for Docx {
    fn add_test_component(self) -> Self {
        self.add_paragraph(Paragraph::new().add_run(Run::new().add_text("this is a test component")))
    }
}