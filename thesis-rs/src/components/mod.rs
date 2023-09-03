pub use self::{
    image::ImageComponent,
    line::LineComponent,
    page_break::PageBreakComponent,
    paragraph::ParagraphComponent,
    placeholder::PlaceholderComponent,
    section_header::SectionHeaderComponent,
    unordered_list::UnorderedListComponent,
};

pub mod image;
pub mod line;
pub mod page_break;
pub mod paragraph;
pub mod placeholder;
pub mod section_header;
pub mod unordered_list;