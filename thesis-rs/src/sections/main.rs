use {
    docx_rs::{Docx, Paragraph, LineSpacing, Run, Tab},
    crate::{
        components::SectionHeaderComponent,
        context::{Context, SectionContext},
    },
};

pub trait MainSection {
    fn add_main_section(self, context: &mut Context) -> Self;
}

impl MainSection for Docx {
    fn add_main_section(self, context: &mut Context) -> Self {
        let section_index = context.next_section_index();
        let mut section_context = SectionContext::new();

        self.add_section_header_placeholder_component(
            format!("{} Аналіз проблеми", section_index).to_uppercase(), 
            "check how this section should be named properly"
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().before(300))
                .style("Heading2")
                .add_run(Run::new().add_tab().add_text(format!("{}.{} ", section_index, section_context.next_subsection_index())).add_text("Ресурси обладнання у розподілених системах та проблема їх ефективного використання"))
        )
    }
}