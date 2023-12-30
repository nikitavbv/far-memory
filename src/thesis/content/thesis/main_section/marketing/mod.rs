use crate::thesis::engine::{Block, section_header, paragraph, SubsectionHeaderBlock, subsection_header, TableBlock, TableCell, Alignment};

pub fn marketing() -> Block {
    Block::Multiple(vec![
        section_header("Маркетинговий аналіз стартап-проекту"),

        subsection_header("Опис ідеї проекту"),
        paragraph("Для опису ідеї проекту проаналізуємо зміст ідеї що пропонується, можливі напрямки застосування, та основні вигоди що може отримати \
користувач цього програмного продукту. Результати аналізу наведено у таблиці 5.1."), // todo: it would be cool to set this number automatically somehow.
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Зміст ідеї".into()).width(3000),
                TableCell::new("Напрямки застосування".into()),
                TableCell::new("Вигоди для користувача".into()),
            ],
            vec![
                vec![
                    TableCell::new("Розробка програмного продукту, що надає віддалену памʼять у розподілених системах".into()).merge_continue(),
                    TableCell::new("Зменшення використання локальної памʼяті за рахунок пересення даних у памʼять віддалених вузлів".into()),
                    TableCell::new("Більш ефективне використання ресурсів центру обробки даних, зниження витрат на обладнання".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("Використання віддаленої памʼяті для роботи з наборами даних що є більшими за обсяг локальної памʼяті".into()),
                    TableCell::new("Підвищення розміру набору даних з яким може працювати програмне забезпечення без значних змін у програмний код".into()),
                ],
            ],
        "Опис ідеї стартап-проекту".to_owned())),

        paragraph("Аналіз техніко-економічних переваг ідеї наведено в таблиці 5.2, де властивості програмного продукту що розглядається порівнюється з \
конкурентами: Carbink та AIFM. Ці програмні продукти є найбільш близькими з технічного боку, але жоден з них не призначений для широкого використання \
у зовнішних центрах обробки даних і не пропонується як продукт чи послуга."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()).merge_continue(),
                TableCell::new("Техніко-економічні характеристики ідеї".into()).merge_continue(),
                TableCell::new("Продукція конкурентів".into()).columns(3),
                TableCell::new("W (слабка сторона)".into()).merge_continue(),
                TableCell::new("N (нейральна сторона)".into()).merge_continue(),
                TableCell::new("S (сильна сторона)".into()).merge_continue(),
            ],
            vec![
                vec![
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("Проект".into()),
                    TableCell::new("Carbink".into()),
                    TableCell::new("AIFM".into()),
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("".into()).merge_continue(),
                ],
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Відкритий вихідниий код та доступність для зовнішнього використання".into()),
                    TableCell::new("+".into()).alignment(Alignment::Center),
                    TableCell::new("-".into()).alignment(Alignment::Center),
                    TableCell::new("+".into()).alignment(Alignment::Center),
                    TableCell::new("".into()).alignment(Alignment::Center),
                    TableCell::new("+".into()).alignment(Alignment::Center),
                    TableCell::new("".into()).alignment(Alignment::Center),
                ],
            ],
            "Аналіз сильних, слабких та нейтральних сторін запропонованої ідеї".to_owned()
        )),

        // TODO: finish this section

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
