use {
    docx_rs::{Docx, Paragraph, Tab, LineSpacing, AlignmentType, Run, Pic},
    crate::thesis::context::Context,
};

pub trait ImageComponent {
    fn add_image_component(self, context: &mut Context, section_index: usize, path: &str, description: &str, scaling: f32, paper_style: bool) -> Self;
}

impl ImageComponent for Docx {
    fn add_image_component(self, context: &mut Context, section_index: usize, path: &str, description: &str, scaling: f32, paper_style: bool) -> Self {
        let img = image::io::Reader::open(path).unwrap().decode().unwrap();

        let width = img.width();
        let height = img.height();

        let width_emu = (5000000 as f32 * scaling) as u32;
        let height_emu = ((height as f32) / (width as f32) * (width_emu as f32)) as u32;

        let image_index = context.next_image_index(section_index);

        let text = if paper_style {
            format!("Fig. {}. {}", image_index, description)
        } else {
            format!("Рисунок {}.{} – {}.", section_index, image_index, description)
        };

        self.add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Center)
                .add_run(
                    Run::new()
                        .add_image(Pic::new(&std::fs::read(path).unwrap()).size(width_emu, height_emu))
                        .add_break(docx_rs::BreakType::TextWrapping)
                        .add_text(text)
                )
        )
    }
}
