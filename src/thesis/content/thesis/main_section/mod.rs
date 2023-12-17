use {
    crate::thesis::engine::{Block, subsection_header, paragraph, unordered_list, section_header, SubsectionHeaderBlock},
    self::{
        research::research,
        method::far_memory_method,
        software::software,
    },
};

mod evaluation;
mod marketing;
mod method;
mod research;
mod software;

pub fn main_section() -> Block {
    Block::Multiple(vec![
        research(),
        far_memory_method(),
        software(),
        evaluation::evaluation(),
        marketing::marketing(),
    ])
}
