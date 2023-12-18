use {
    crate::thesis::engine::{Block, subsection_header, paragraph, SubsectionHeaderBlock, TextSpan},
    super::super::applications::{DEPLOYMENT_DIAGRAM, COMPONENT_DIAGRAM},
};

mod requirements;
mod tools;

pub fn software() -> Block {
    Block::Multiple(vec![
        requirements::requirements(),
        tools::tools(),

        subsection_header("Архітектура програмного забезпечення"),
        subsection_header(SubsectionHeaderBlock::new("Компоненти програмного забезпечення що надає віддалену памʼять".to_owned()).with_level(2)),
        paragraph("Як було зазначено раніше, цей метод надання віддаленої памʼяті використовує три компоненти: інтеграція у програмне забезпечення на \
стороні вузлів обчислення, вузли зберігання та вузел керування. "),
        paragraph(TextSpan::Multiple(vec![
            "Схема структурна розгортання цих компонентів наведена у додатку ".into(),
            TextSpan::ApplicationReference(DEPLOYMENT_DIAGRAM),
            ".".into(),
        ])),
        paragraph("Інтеграція у програмне забезпечення представлена клієнтською бібліотекою або віртуальним блоковим присторєм. Інтеграція налаштовується \
розробником інформаційної системи яка використовує віддалену памʼять. У разі використання бібліотеки, цей компонент розгортається у кількості екземплярів \
рівній кількості екземплярів програмного забезпечення інформаційної системи. При використанні блокового пристроя - в залежності від кількості серверів \
на яких розміщена інформаційна система та кількості блокових пристроїв на кожному з них. Очікується що під час роботи інформаційної системи кількість \
вузлів обчислень може змінюватись."),
        paragraph("Вузел керування та вузли зберігання розгортаються на інших серверах. Вузел керування завжди один, а кількість вузлів зберігання \
може змінюватись адміністратором інформаційної системи або планувальником задач, що використовується, в залежності від обсягу вільних ресурсів. На \
одному сервері може одночасно бути розгорнуто декілька вузлів зберігання."),

        subsection_header(SubsectionHeaderBlock::new("Взаємодія компонентів".to_owned()).with_level(2)),
        paragraph(TextSpan::Multiple(vec![
            "Схема структурна компонентів програмного забезпечення що надає віддалену памʼять наведено у додатку ".into(),
            TextSpan::ApplicationReference(COMPONENT_DIAGRAM),
            ". Ця схема окрім компонентів також показує звʼязки між ними, а саме:".into(),
        ])),
        Block::UnorderedList(vec![
            "вузли зберігання передають ...".to_owned(),
        ]),

        subsection_header(SubsectionHeaderBlock::new("Структура клієнта віддаленої памʼяті".to_owned()).with_level(2)),
        // - class diagram - (add for far memory manager and storage later if I will need more pages.)
        // TODO: brief description here

        subsection_header(SubsectionHeaderBlock::new("Послідовність доступу до даних у віддаленій памʼяті".to_owned()).with_level(2)),
        // - sequence diagram
        // TODO: brief description here

        subsection_header(SubsectionHeaderBlock::new("Послідовність роботи фонового потоку клієнта віддаленої памʼяті".to_owned()).with_level(2)),
        // - sequence diagram
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
