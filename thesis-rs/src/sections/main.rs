use {
    docx_rs::Docx,
    crate::components::SectionHeaderComponent,
};

pub trait MainSection {
    fn add_main_section(self) -> Self;
}

impl MainSection for Docx {
    fn add_main_section(self) -> Self {
        self.add_section_header_placeholder_component(
            "Розділ І. Аналіз проблеми".to_uppercase(), 
            "check how this section should be named properly"
        )
    }
}