use crate::engine::Block;

pub fn documentation() -> Block {
    Block::Multiple(vec![
        Block::SectionHeader("far memory".to_owned()),
        Block::Note(r#"Please note that most parts of documentation for this project are in Ukrainian because I am working on this in scope of my thesis at Kyiv Polytechnic Institute and I
need to be able to refer to this documentation when talking to thesis supervisors and other people from the university. I will probably add English translation later."#.to_owned()),
        Block::SubsectionHeader("Віддалена памʼять".to_owned()),
        Block::Placeholder(Box::new(Block::Paragraph("Віддалена памʼять - тип памʼяті що знаходиться на віддалених вузлах у розподіленій системі.".to_owned())), "add text explaining what it is".to_owned()),
    ])
}