use {
    docx_rs::Docx,
    crate::{
        sections::{            
            FrontPageSection, 
            TaskSection, 
            AbstractSection, 
            TableOfContentsSection,
            AbbreviationsListSection,
            IntroSection,
            MainSection,
            ConclusionsSection,
            ReferencesSection,
        },
        context::Context,
        content::{Content, Language},
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
            .add_table_of_contents_section()
            .add_abbreviations_list_section()
            .add_intro_section(context)
            .add_main_section(context, &content)
            .add_conclusions_section()
            .add_references_section(context)
    }
}