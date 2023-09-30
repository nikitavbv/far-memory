use docx_rs::{Run, Pic};

pub trait LineComponent {
    fn add_line_component(self, width: u32) -> Self;
}

impl LineComponent for Run {
    fn add_line_component(self, width: u32) -> Self {
        self.add_image(Pic::new(&std::fs::read("images/line.gif").unwrap()).size(width, 17000))
    }
}
