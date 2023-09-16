use {
    docx_rs::Docx,
    crate::{
        context::Context,
        content::Content,
        engine::render_blocks_to_docx,
    },
};

pub trait MainSection {
    fn add_main_section(self, context: &mut Context, content: &Content) -> Self;
}

impl MainSection for Docx {
    fn add_main_section(self, context: &mut Context, content: &Content) -> Self {
        render_blocks_to_docx(self, context, content.main.clone())
    }
}