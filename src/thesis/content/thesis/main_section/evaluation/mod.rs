use crate::thesis::engine::{Block, section_header, paragraph, SubsectionHeaderBlock};

pub fn evaluation() -> Block {
    Block::Multiple(vec![
        section_header("Оцінка ефективності"), // todo: how to name it correctly?
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
