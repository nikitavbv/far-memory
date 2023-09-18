use {
    docx_rs::Docx,
    crate::{
        sections::{            
            FrontPageSection, 
            TaskSection, 
            AbstractSection, 
        },
        context::Context,
        content::{Content, Language},
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
            .add_abstract_section(context, &content, &Language::Ukrainian)
            .add_abstract_section(context, &content, &Language::English)
            .add_text_block(context, content.main.clone())
    }
}