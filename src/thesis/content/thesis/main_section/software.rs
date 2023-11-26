use crate::thesis::engine::{Block, subsection_header, section_header, paragraph, SubsectionHeaderBlock};

pub fn software() -> Block {
    Block::Multiple(vec![
        section_header("Опис програмного забезпечення"),
        subsection_header("Вимоги до програмного забезпечення"),
        paragraph("Як було зазначено в попередніх розділах, до реалізації віддаленої памʼяті, що розглядається в цій роботі, висуваються \
вимоги які визначені розглянутими особливостями середовища та програмного забезпечення, у яке вона інтегрується."),
        paragraph("Віддалена памʼять повинна інтегруватися у програмне забезпечення за допомогою бібліотеки або віртуального блокового \
пристрою. У разі використання бібліотеки, розробнику прикладного програмного забезпечення повинен надаватись клієнт віддаленої памʼяті, \
який надає засоби для зберігання послідовностей байт, обʼєктів наданих користувачем та структур даних оптимізованих для роботи з віддаленою \
памʼяттю."),
        paragraph("Реалізація віддаленої памʼяті повинна коректно обробляти події виходу з ладу віддалених вузлів зберігання та підтримувати \
запланований вивід вузлів на обслуговування, що є типовою вимогою для програмного забезпечення що працює у розподіленій системі. У разі \
виходу вузла зберігання з ладу, клієнт віддаленої памʼяті повинен мінімізувати вирогідність втрати даних через їх відновлення з памʼяті \
інших вузлів."),
        // todo: move to next requirements (performance). tell about hardware requirements and tell that software needs to be easy to use.
        // requirements: I already told about requirements in previous section, but it makes sense to revise them briefly and talk a bit about hardware requirements as well.
        // вимоги до програмного продукту
        // - вимоги до функціональних характеристик
        // - вимоги до надійності
        // - вимоги до складу і параметрів технічних засобів

        subsection_header("Засоби розробки"),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
        // засоби розробки
        // tell about Rust and some main crates that I use and why.

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
