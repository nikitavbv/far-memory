use {
    crate::thesis::engine::{Block, empty_block},
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

pub fn main_section(include_marketing_section: bool) -> Block {
    Block::Multiple(vec![
        research(),
        far_memory_method(),
        software(),
        evaluation::evaluation(),
        if include_marketing_section { marketing::marketing() } else { empty_block() },
    ])
}
