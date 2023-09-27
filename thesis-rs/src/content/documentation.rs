use crate::engine::Block;

pub fn documentation() -> Block {
    Block::Multiple(vec![
        Block::SectionHeader("far memory".to_owned()),
        Block::Note(r#"Please note that most parts of documentation for this project are in Ukrainian because I am working on this in scope of my thesis at Kyiv Polytechnic Institute and I
need to be able to refer to this documentation when talking to thesis supervisors and other people from the university. I will probably add English translation later."#.to_owned()),
        Block::Paragraph("some text here".to_owned()),
    ])
}