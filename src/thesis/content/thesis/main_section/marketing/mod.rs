use crate::thesis::engine::{Block, section_header, paragraph, SubsectionHeaderBlock, subsection_header, TableBlock, TableCell, Alignment};

pub fn marketing() -> Block {
    let comparison_table_sign_width = 5000;
    let comparison_table_property_width = 500;

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
                TableCell::new("№".into()).merge_continue().font_size(12).width(100),
                TableCell::new("Техніко-економічні характеристики ідеї".into()).merge_continue().font_size(12).width(comparison_table_property_width),
                TableCell::new("Продукція конкурентів".into()).columns(3).font_size(12).width(3 * comparison_table_sign_width),
                TableCell::new("W (слабка сторона)".into()).merge_continue().font_size(12).width(comparison_table_sign_width),
                TableCell::new("N (нейтральна сторона)".into()).merge_continue().font_size(12).width(comparison_table_sign_width),
                TableCell::new("S (сильна сторона)".into()).merge_continue().font_size(12).width(comparison_table_sign_width),
            ],
            vec![
                vec![
                    TableCell::new("".into()).merge_continue().width(100),
                    TableCell::new("".into()).merge_continue().width(comparison_table_property_width),
                    TableCell::new("Проект".into()).font_size(12).width(comparison_table_sign_width),
                    TableCell::new("Carbink".into()).font_size(12).width(comparison_table_sign_width),
                    TableCell::new("AIFM".into()).font_size(12).width(comparison_table_sign_width),
                    TableCell::new("".into()).merge_continue().width(comparison_table_sign_width),
                    TableCell::new("".into()).merge_continue().width(comparison_table_sign_width),
                    TableCell::new("".into()).merge_continue().width(comparison_table_sign_width),
                ],
                vec![
                    TableCell::new("1".into()).font_size(12).width(100),
                    TableCell::new("Відкритий вихідниий код та доступність для зовнішнього використання".into()).font_size(12).width(comparison_table_property_width),
                    TableCell::new("+".into()).alignment(Alignment::Center).width(comparison_table_sign_width),
                    TableCell::new("-".into()).alignment(Alignment::Center).width(comparison_table_sign_width),
                    TableCell::new("+".into()).alignment(Alignment::Center).width(comparison_table_sign_width),
                    TableCell::new("".into()).alignment(Alignment::Center).width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                ],
                vec![
                    TableCell::new("2".into()).font_size(12).width(100),
                    TableCell::new("Не залежить від спеціалізованого апаратного забезпечення".into()).font_size(12).width(comparison_table_property_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_minus().width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                ],
                vec![
                    TableCell::new("3".into()).font_size(12).width(100),
                    TableCell::new("Зберігання даних на багатьох віддалених вузлах".into()).font_size(12).width(comparison_table_property_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_minus().width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                ],
                vec![
                    TableCell::new("4".into()).font_size(12).width(100),
                    TableCell::new("Підтримка інтеграції без зміни коду".into()).font_size(12).width(comparison_table_property_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_minus().width(comparison_table_sign_width),
                    table_cell_minus().width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                ],
                vec![
                    TableCell::new("5".into()).font_size(12).width(100),
                    TableCell::new("Зниження затримки за рахунок керування заміщенням сторінок".into()).font_size(12).width(comparison_table_property_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                    table_cell_empty().width(comparison_table_sign_width),
                    table_cell_plus().width(comparison_table_sign_width),
                ],
            ],
            "Аналіз сильних, слабких та нейтральних сторін запропонованої ідеї".to_owned()
        ).with_split(vec![3])),

        paragraph("Згідно з наведеною таблицею, програмний продукт що розглядається має переваги перед потенційними конкурентами, а саме: відкритий \
програмний код та доступність для зовнішнього використання, легка інтеграція у програмне забезпечення та зниження затримки доступу за рахунок більш \
ефективного алгоритму заміщення проміжків."),

        subsection_header("Технологійчний аудит ідеї проекту"),
        paragraph("Для проведення технологічного аудиту ідеї проекту було проаналізовано які технології є необхідними для реалізації ідеї цього проекту. \
У таблиці 5.3. проведено аналіз технологійчної здійсненності ідеї проекту."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Ідея проекту".into()),
                TableCell::new("Технології її реалізації".into()),
                TableCell::new("Наявність технологій".into()),
                TableCell::new("Доступність технологій".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Передача даних між вузлами без використання спеціалізованого апаратного забезпечення".into()),
                    TableCell::new("Бінарний протокол на основі TCP".into()),
                    table_cell_plus(),
                    table_cell_plus(),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Відстеження доступу до даних за допомогою розумних показчиків та лічильника посилань".into()),
                    TableCell::new("Властивності AsRef, Drop у мові програмування Rust, а також структура AtomicU64".into()),
                    table_cell_plus(),
                    table_cell_plus(),
                ],
                vec![
                    TableCell::new("3".into()),
                    // TODO: virtual block device
                ],
                // TODO: erasure coding
                // TODO: stats analysis
            ],
            "Технологічна здійсненність ідеї проекту".to_owned(),
        )),

        subsection_header("Аналіз ринкових можливостей запуску стартап-проекту"),
        // TODO: add this subsection

        subsection_header("Розроблення ринкової стратегії проекту"),
        // TODO: add this subsection

        subsection_header("Розроблення маркетингової програми стартап-проєкту"),
        // TODO: add this subsection

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        // TODO: add conclusions
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
