use crate::thesis::engine::{Block, section_header, paragraph, SubsectionHeaderBlock, subsection_header, TableBlock, TableCell, Alignment, TextSpan};

pub fn marketing() -> Block {
    let comparison_table_sign_width = 5000;
    let comparison_table_property_width = 500;

    let technology_table_number_width = 100;
    let technology_table_description_width = 2700;
    let technology_table_proprerty_width = 2000;

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
У таблиці 5.3 проведено аналіз технологійчної здійсненності ідеї проекту."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()).width(technology_table_number_width),
                TableCell::new("Ідея проекту".into()).width(technology_table_description_width),
                TableCell::new("Технології її реалізації".into()).width(technology_table_description_width),
                TableCell::new("Наявність технологій".into()).width(technology_table_proprerty_width),
                TableCell::new("Доступність технологій".into()).width(technology_table_proprerty_width),
            ],
            vec![
                vec![
                    TableCell::new("1".into()).width(technology_table_number_width),
                    TableCell::new("Передача даних між вузлами без використання спеціалізованого апаратного забезпечення".into()).width(technology_table_description_width),
                    TableCell::new("Бінарний протокол на основі TCP".into()).width(technology_table_description_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                ],
                vec![
                    TableCell::new("2".into()).width(technology_table_number_width),
                    TableCell::new("Відстеження доступу до даних за допомогою розумних показчиків та лічильника посилань".into()).width(technology_table_description_width),
                    TableCell::new("Властивності AsRef, Drop у мові програмування Rust, а також структура AtomicU64".into()).width(technology_table_description_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                ],
                vec![
                    TableCell::new("3".into()).width(technology_table_number_width),
                    TableCell::new("Надання віддаленої памʼяті за допомогою віртуального блокового пристрою".into()).width(technology_table_description_width),
                    TableCell::new("Модуль Network Block Device ядра операційної системи Linux".into()).width(technology_table_description_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                ],
                vec![
                    TableCell::new("4".into()).width(technology_table_number_width),
                    TableCell::new("Кодування стиранням проміжків у віддаленій памʼяті".into()).width(technology_table_description_width),
                    TableCell::new("Бібліотека reed-solomon-erasure у мові програмування Rust".into()).width(technology_table_description_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                ],
                vec![
                    TableCell::new("5".into()).width(technology_table_number_width),
                    TableCell::new("Прогнозування доступу до проміжків даних за допомогою рекурентної нейнронної мережі".into()).width(technology_table_description_width),
                    TableCell::new("Бібліотека машинного навчання candle у мові програмування Rust".into()).width(technology_table_description_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                    table_cell_plus().width(technology_table_proprerty_width),
                ],
                vec![
                    TableCell::new(TextSpan::Italic(Box::new(TextSpan::Regular("Обрані технології для реалізації ідеї проекту: 1, 2, 3, 4, 5.".into())))).columns(5),
                ],
            ],
            "Технологічна здійсненність ідеї проекту".to_owned(),
        ).with_split(vec![2])),
        paragraph("Виходячи з проведеного аналізу, технологічна здійсненість проєкту можлива, найбільш прийнятні технології, що було обрані, є наявними та \
доступними."),

        subsection_header("Аналіз ринкових можливостей запуску стартап-проекту"),
        paragraph(vec![
            "Першим кроком визначення ринкових можливостей, які можна використати під час ринкового впровадження проекту, є аналіз попиту. \
У таблиці 5.4 проведено попередню характеристику потенційного ринку стратап-проекту.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Показники стану ринку".into()),
                TableCell::new("Характеристика".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Кількість головних гравців, од.".into()),
                    TableCell::new("0".into()),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Загальний обсяг продаж, грн/ум.од".into()),
                    TableCell::new("Точна статистика відсутня. Розмір ринку програмного забезпечення для надання віддаленої памʼяті може приблизно \
дорівнювати розміру ринку віртуалізації ($86 мільядрів), оскільки потреба в використанні віддаленої памʼяті виникає у схожих середовищах.".into()),
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("Динаміка ринку (якісна оцінка)".into()),
                    TableCell::new("Зростає".into()),
                ],
                vec![
                    TableCell::new("4".into()),
                    TableCell::new("Наявність обмежень для входу (вказати характер обмежень)".into()),
                    TableCell::new("Немає".into()),
                ],
                vec![
                    TableCell::new("5".into()),
                    TableCell::new("Специфічні вимоги до стандартизації та сертифікації".into()),
                    TableCell::new("Немає".into()),
                ],
                vec![
                    TableCell::new("6".into()),
                    TableCell::new("Середня норма рентабельності в галузлі (або по ринку), %".into()),
                    TableCell::new("Невідома".into()),
                ],
            ],
            "Попередня характеристика потенційного ринку стратап-проекту".to_owned(),
        )),
        paragraph("Незважаючи на відсутність повної інформації про потенційний ринок (зумовлену відсутністю існуючих комерційних програмних продуктів у \
цій сфері), можна зробити висновок що цей ринок є привабливим через велику кількість клієнтів (розмір ринку) які зацікавлені в збільшенні ефективності \
використання ресурсів у центрі обробки даних, відсутність конкурентів у цій сфері та відсутність обмежень для входу."),

        paragraph(vec![
            "Основна характеристика майбутніх клієнтів наведена у таблиці 5.5.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Потреба, що формує ринок".into()),
                TableCell::new("Цільова аудиторія (цільові сегменти ринку)".into()),
                TableCell::new("Відмінності у поведінці різних потенційних цільових груп клієнтів".into()),
                TableCell::new("Вимоги споживачів до товару".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Збільшення ефективності використання ресурсів центру обробки даних".into()),
                    TableCell::new("Оператори великих центрів обробки даних".into()),
                    TableCell::new("відсутні".into()),
                    TableCell::new(vec![
                        "Легкість у розгортанні".into(),
                        TextSpan::Break,
                        "Легкість у інтеграції у програмне забезпечення".into(),
                        TextSpan::Break,
                        "Рівень відмовостійкості що є допустимим для інформаційної системи, в яку інтегрується віддалена памʼять".into(),
                        TextSpan::Break,
                        "Рівень швидкодії що дозволяє інформаційнії системі, в яку інтегрується віддалена памʼять, дотримуватись цільового рівня обслуговування (SLO)".into(),
                    ].into()),
                ],
            ],
            "Характеристика потенційних клієнтів стартап-проекту".to_owned(),
        )),

        paragraph("Важливо розглянути фактори що можуть перешкоджати ринковому впровадженню проекту. Такі фактори розглянуто у таблиці 5.6."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Фактор".into()),
                TableCell::new("Зміст загрози".into()),
                TableCell::new("Можлива реакція компанії".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Конкурентне середовище".into()),
                    TableCell::new("Вирішення задачі збільшення ефективності використання оперативної памʼяті на рівні віртуалізації або інших компонентів \
хмарної інфраструктури.".into()),
                    TableCell::new("Підвищення ефективності програмного продукту (рівня швидкодії), оскільки інтеграція на рівні програмного забезпечення \
повинна бути більш ефективною через більшу кількість наявної інформації.".into()),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Незацікавленість клієнтів у використанні програмного продукту".into()),
                    TableCell::new("".into()), // TODO
                    TableCell::new("".into()), // TODO
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("Відповідність продукту очікванням та потребам сегменту".into()),
                    TableCell::new("".into()), // TODO
                    TableCell::new("".into()), // TODO
                ],
            ],
            "Фактори загроз".to_owned(),
        )),

        // TODO: tell something about risks (TODO: verify)
        // TODO: table to analyze risks (TODO: verify)

        // TODO: tell something about opportunities (TODO: verify)
        // TODO: table to analyze opportunities (TODO: verify)

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
