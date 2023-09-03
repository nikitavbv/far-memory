pub use self::{
    abbreviations::AbbreviationsListSection,
    abstract_section::AbstractSection,
    conclusions::ConclusionsSection,
    front_page::FrontPageSection,
    references::ReferencesSection,
    table_of_contents::TableOfContentsSection,
    task::TaskSection,
    intro::IntroSection,
    main::MainSection,
};

pub mod abbreviations;
pub mod abstract_section;
pub mod conclusions;
pub mod front_page;
pub mod intro;
pub mod main;
pub mod references;
pub mod table_of_contents;
pub mod task;