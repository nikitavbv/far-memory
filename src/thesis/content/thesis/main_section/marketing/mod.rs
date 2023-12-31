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
                TableCell::new("№".into()).merge_continue().font_size(12),
                TableCell::new("Техніко-економічні характеристики ідеї".into()).merge_continue().font_size(12),
                TableCell::new("Продукція конкурентів".into()).columns(3).font_size(12),
                TableCell::new("W (слабка сторона)".into()).merge_continue().font_size(12),
                TableCell::new("N (нейтральна сторона)".into()).merge_continue().font_size(12),
                TableCell::new("S (сильна сторона)".into()).merge_continue().font_size(12),
            ],
            vec![
                vec![
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("Проект".into()).font_size(12),
                    TableCell::new("Carbink".into()).font_size(12),
                    TableCell::new("AIFM".into()).font_size(12),
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
                    table_cell_empty(),
                    table_cell_plus(),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Не залежить від спеціалізованого апаратного забезпечення".into()),
                    table_cell_plus(),
                    table_cell_plus(),
                    table_cell_minus(),
                    table_cell_empty(),
                    table_cell_plus(),
                    table_cell_empty(),
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("Зберігання даних на багатьох віддалених вузлах".into()),
                    table_cell_plus(),
                    table_cell_plus(),
                    table_cell_minus(),
                    table_cell_empty(),
                    table_cell_plus(),
                    table_cell_empty(),
                ],
                vec![
                    TableCell::new("4".into()),
                    TableCell::new("Підтримка інтеграції без зміни коду".into()),
                    table_cell_plus(),
                    table_cell_minus(),
                    table_cell_minus(),
                    table_cell_empty(),
                    table_cell_empty(),
                    table_cell_plus(),
                ],
                vec![
                    TableCell::new("5".into()),
                    TableCell::new("Зниження затримки за рахунок керування заміщенням сторінок".into()),
                    table_cell_plus(),
                    table_cell_plus(),
                    table_cell_plus(),
                    table_cell_empty(),
                    table_cell_empty(),
                    table_cell_plus(),
                ],
            ],
            "Аналіз сильних, слабких та нейтральних сторін запропонованої ідеї".to_owned()
        ).with_split(vec![2])),

        // TODO: finish this section

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}

fn table_cell_plus() -> TableCell {
    TableCell::new("+".into()).alignment(Alignment::Center)
}

fn table_cell_minus() -> TableCell {
    TableCell::new("-".into()).alignment(Alignment::Center)
}

fn table_cell_empty() -> TableCell {
    TableCell::new("".into()).alignment(Alignment::Center)
}
