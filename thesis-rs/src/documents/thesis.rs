use {
    docx_rs::Docx,
    crate::{
        context::Context,
        content::Content,
        engine::TextBlockComponent,
    },
};

pub trait ThesisDocument {
    fn add_thesis_document(self, context: &mut Context, content: &Content) -> Self;
}

impl ThesisDocument for Docx {
    fn add_thesis_document(self, context: &mut Context, content: &Content) -> Self {
        self
            .add_text_block(context, &content, content.main.clone())
    }
}