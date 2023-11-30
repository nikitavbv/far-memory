use crate::thesis::engine::{Block, subsection_header, section_header, paragraph, SubsectionHeaderBlock};

mod requirements;
mod tools;

pub fn software() -> Block {
    Block::Multiple(vec![
        requirements::requirements(),
        tools::tools(),

        subsection_header("Архітектура програмного забезпечення"),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
        // архітектура програмного забезпечення
        // - схема структура розгортання
        // - діаграма класів (for far memory client. For far memory manager and storage?)
        // - діаграма послідовностей (data structure access flow, background swap out/swap in flow).
        // - діаграма компонентів (how services communicate).
        // - специфікація функцій (only add if I need some more pages, lol).

        subsection_header("Інструкція користувача"),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
        // інструкція користувача
        // tell how users are expected to install and operate far memory. tell a bit about deployment as well. tell about options to use Kubernetes.

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
