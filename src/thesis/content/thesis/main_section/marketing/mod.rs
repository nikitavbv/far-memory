use crate::thesis::engine::{Block, section_header, paragraph, SubsectionHeaderBlock, subsection_header, TableBlock, TableCell};

pub fn marketing() -> Block {
    Block::Multiple(vec![
        section_header("Маркетинговий аналіз стартап-проекту"),

        subsection_header("Опис ідеї проекту"),
        paragraph("Для опису ідеї проекту проаналізуємо зміст ідеї що пропонується, можливі напрямки застосування, та основні вигоди що може отримати \
користувач цього програмного продукту. Результати аналізу наведено у наступній таблиці."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Зміст ідеї".into()),
                TableCell::new("Напрямки застосування".into()),
                TableCell::new("Вигоди для користувача".into()),
            ],
            vec![
                vec![
                    TableCell::new("Розробка програмного продукту, що надає віддалену памʼять у розподілених системах".into()).merge_continue(),
                    TableCell::new("Зменшення використання локальної памʼяті за рахунок пересення даних у памʼять віддалених вузлів.".into()),
                    TableCell::new("Більш ефективне використання ресурсів центру обробки даних, зниження витрат на обладнання".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue(), // todo: continue somehow
                    TableCell::new("Використання віддаленої памʼяті для роботи з наборами даних що є більшими за обсяг локальної памʼяті".into()),
                    TableCell::new("Підвищення розміру набору даних з яким може працювати програмне забезпечення без значних змін у програмний код".into()),
                ],
            ],
        "Опис ідеї стартап-проекту".to_owned())),

        // TODO: finish this section

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
