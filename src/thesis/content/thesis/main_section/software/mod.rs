use crate::thesis::engine::{Block, subsection_header, paragraph, SubsectionHeaderBlock};

mod requirements;
mod tools;

pub fn software() -> Block {
    Block::Multiple(vec![
        requirements::requirements(),
        tools::tools(),

        subsection_header("Архітектура програмного забезпечення"),
        // - Компоненти програмного забезпечення що надає віддалену памʼять - deployment diagram - show components and how they communicate
        paragraph("Як було зазначено раніше, цей метод надання віддаленої памʼяті використовує три компоненти: інтеграція у програмне забезпечення на \
стороні вузлів обчислення, вузли зберігання та вузел керування. "),

        // - Взаємодія компонентів - component diagram - show which data is passed where
        // TODO: add some description here

        // - Структура компоненту клієнта віддаленої памʼяті - class diagram - (add for far memory manager and storage later if I will need more pages.)
        // TODO: brief description here

        // - Послідовність доступу до даних у віддаленій памʼяті - sequence diagram
        // TODO: brief description here

        // - Послідовність роботи фонового потоку клієнта віддаленої памʼяті - sequence diagram
        // TODO: brief description here

        // - специфікація функцій (only add if I need some more pages, lol).

        subsection_header("Інструкція користувача"),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
        // інструкція користувача
        // tell how users are expected to install and operate far memory. tell a bit about deployment as well. tell about options to use Kubernetes.

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
