use {
    docx_rs::Docx,
    crate::{
        sections::{            
            FrontPageSection, 
            TaskSection, 
        },
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
            .add_front_page_section(&content)
            .add_task_section(context, &content)
            .add_text_block(context, &content, content.main.clone())
    }
}