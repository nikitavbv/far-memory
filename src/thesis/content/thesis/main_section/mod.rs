use {
    crate::thesis::engine::{Block, subsection_header, paragraph, unordered_list, section_header, SubsectionHeaderBlock},
    self::{
        research::research,
        method::far_memory_method,
        software::software,
    },
};

mod method;
mod research;
mod software;

pub fn main_section() -> Block {
    Block::Multiple(vec![
        research(),
        far_memory_method(),
        software(),

        section_header("Оцінка ефективності"), // todo: how to name it correctly?
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
        // in third section explain that a demo app was implemented to measure how well everything works and the hardware of the test environment.

        section_header("Маркетинговий аналіз стартап-проекту"),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
