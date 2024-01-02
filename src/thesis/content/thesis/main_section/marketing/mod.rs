use crate::thesis::engine::{Block, section_header, paragraph, SubsectionHeaderBlock, subsection_header, TableBlock, TableCell, Alignment, TextSpan};

pub fn marketing() -> Block {
    let comparison_table_sign_width = 5000;
    let comparison_table_property_width = 500;

    let technology_table_number_width = 100;
    let technology_table_description_width = 2700;
    let technology_table_proprerty_width = 2000;

    let risks_table_number_width = 500;
    let risks_table_factor_width = 2500;
    let risks_table_description_width = 3000;
    let risks_table_solution_width = 3500;

    let opportunities_table_number_width = 500;
    let opportunities_table_text_width = 3000;

    let competition_table_number_width = 500;
    let competition_table_property_width = 2000;
    let competition_table_description_width = 3500;

    let product_levels_table_property_width = 500;

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
                        "Рівень швидкодії що дозволяє інформаційній системі, в яку інтегрується віддалена памʼять, дотримуватись цільового рівня обслуговування (SLO)".into(),
                    ].into()),
                ],
            ],
            "Характеристика потенційних клієнтів стартап-проекту".to_owned(),
        )),

        paragraph("Важливо розглянути фактори що можуть перешкоджати ринковому впровадженню проекту. Такі фактори розглянуто у таблиці 5.6."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()).width(risks_table_number_width),
                TableCell::new("Фактор".into()).width(risks_table_factor_width),
                TableCell::new("Зміст загрози".into()).width(risks_table_description_width),
                TableCell::new("Можлива реакція компанії".into()).width(risks_table_solution_width),
            ],
            vec![
                vec![
                    TableCell::new("1".into()).width(risks_table_number_width),
                    TableCell::new("Конкурентне середовище".into()).width(risks_table_factor_width),
                    TableCell::new("Вирішення задачі збільшення ефективності використання оперативної памʼяті на рівні віртуалізації або інших компонентів \
хмарної інфраструктури.".into()).width(risks_table_description_width),
                    TableCell::new("Підвищення ефективності програмного продукту (рівня швидкодії), оскільки інтеграція на рівні програмного забезпечення \
повинна бути більш ефективною через більшу кількість наявної інформації.".into()).width(risks_table_solution_width),
                ],
                vec![
                    TableCell::new("2".into()).width(risks_table_number_width),
                    TableCell::new("Незацікавленість клієнтів у використанні програмного продукту".into()).width(risks_table_factor_width),
                    TableCell::new("Клієнти зацікавлені у використанні інших підходів до підвищення ефективності використання ресурсів та вважають \
використання віддаленої памʼяті недоцільним.".into()).width(risks_table_description_width),
                    TableCell::new("Спрощення розгортання компонентів віддаленої памʼяті, вдосконалення методу інтеграції у програмне забезпечення, \
покращення рівня швидкодії що збільшить область застосування.".into()).width(risks_table_solution_width),
                ],
                vec![
                    TableCell::new("3".into()).width(risks_table_number_width),
                    TableCell::new("Відповідність продукту очікванням та потребам сегменту".into()).width(risks_table_factor_width),
                    TableCell::new("Рівень швидкодії віддаленої памʼяті не дозволяє інформаціним системам, в які віддалена памʼять інтегрується, дотримуватись \
цільового рівня обслуговування (SLO)".into()).width(risks_table_description_width),
                    TableCell::new("Зниження затримки доступу до даних у віддаленій памʼяті за рахунок збільшення ефективності клієнта віддаленої памʼяті, \
використання більш ефективних алгоритмів заміщення проміжків, і т.д.".into()).width(risks_table_solution_width),
                ],
            ],
            "Фактори загроз".to_owned(),
        ).with_split(vec![2])),

        paragraph("Крім цього, існують фактори що сприяють ринковому впровадженню проекту. Ці фактори розглянуто у таблиці 5.7."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()).width(opportunities_table_number_width),
                TableCell::new("Фактор".into()).width(opportunities_table_text_width),
                TableCell::new("Зміст можливості".into()).width(opportunities_table_text_width),
                TableCell::new("Можлива реакція компанії".into()).width(opportunities_table_text_width),
            ],
            vec![
                vec![
                    TableCell::new("1".into()).width(opportunities_table_number_width),
                    TableCell::new("Підвищення цін на серверне обладнання, виникнення складнощів з його покупкою у операторів центрів обробки даних".into()).width(opportunities_table_text_width),
                    TableCell::new("Необхідність оптимізації використання ресурсів стає більш критичною для клієнтів".into()).width(opportunities_table_text_width),
                    TableCell::new("Рекламна компанія спрямована на ознайомлення клієнтів з можливістю використання віддаленої памʼятті.".into()).width(opportunities_table_text_width),
                ],
                vec![
                    TableCell::new("2".into()).width(opportunities_table_number_width),
                    TableCell::new("Значні зміни на ринку віртуалізації/хмарних обчислень".into()).width(opportunities_table_text_width),
                    TableCell::new("Умови в яких розробники інформаційних систем переглядають архітектуру програмного забезпечення, інтеграція віддаленої \
памʼяті може бути однією зі змін що виконуються одночасно.".into()).width(opportunities_table_text_width),
                    TableCell::new("Надання послуг консультування для таких міграцій".into()).width(opportunities_table_text_width),
                ],
                vec![
                    TableCell::new("3".into()).width(opportunities_table_number_width),
                    TableCell::new("Можливості демонстрації програмного продукту".into()).width(opportunities_table_text_width),
                    TableCell::new("Популярне програмне забезпечення використовує багато оперативної памʼяті (наприклад, бази даних)".into()).width(opportunities_table_text_width),
                    TableCell::new("Для програмного забезпечення з відкритим кодом створювати версії в які інтегрована віддалена памʼять та яке є простим \
у розгортанні. Виконати вимірювання швидкодії для демонстрації привабливості використання віддаленої памʼяті.".into()).width(opportunities_table_text_width),
                ],
            ],
            "Фактори можливостей".to_owned(),
        ).with_split(vec![2])),

        paragraph(vec![
            "Для більш ефективного розвитку програмного продукту на ринку необхідно провести аналіз рис конкуренції на ринку. Ступеневий аналіз \
конкуренції на ринку розглянуто у таблиці 5.8.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()).width(competition_table_number_width),
                TableCell::new("Особливості конкурентного середовища".into()).width(competition_table_property_width),
                TableCell::new("В чому проявляється дана характеритстика".into()).width(competition_table_description_width),
                TableCell::new("Вплив на діяльність підприємства (можливі дії компанії, щоб бути конкурентноспроможною)".into()).width(competition_table_description_width),
            ],
            vec![
                vec![
                    TableCell::new("1".into()).width(competition_table_number_width),
                    TableCell::new("Тип конкуренції: чиста".into()).width(competition_table_property_width),
                    TableCell::new("Програмний продукт для забезпечення віддаленої памʼяті не може повністю замінити інше програмне забезпечення для \
надання віддаленої памʼяті, можуть співіснувати навіть у межах одного центру обробки даних; Ринок має умови для входу і виходу; Ціна відрізняється між \
конкурентами.".into()).width(competition_table_description_width),
                    TableCell::new("Розробка програмного продукту з властивостями, яких немає у конкурентів. Встановлення ціни що відображає рівень переваг \
над інишими програмними продуктами.".into()).width(competition_table_description_width),
                ],
                vec![
                    TableCell::new("2".into()).width(competition_table_number_width),
                    TableCell::new("За рівнем конкурентної боротьби: мультинаціональний бізнес".into()).width(competition_table_property_width),
                    TableCell::new("Програмний продукт не потребує значних змін для використання у закордонних центрах обробки даних.".into()).width(competition_table_description_width),
                    TableCell::new("Використання великої кількості програмного забезпечення для інтеграції з усього світу. Орієнтація на інтеграцію у \
найбільші світові інформаційні системи та центри обробки даних та використання цього для подальшого розвитку продукту.".into()).width(competition_table_description_width),
                ],
                vec![
                    TableCell::new("3".into()).width(competition_table_number_width),
                    TableCell::new("За галузевою ознакою: міжгалузева".into()).width(competition_table_property_width),
                    TableCell::new("Конкуренція з іншими типами рішень: на рівні інфраструктури чи апаратних компонентів які також могли б вирішувати \
проблему більш ефективного використання оперативної памʼяті".into()).width(competition_table_description_width),
                    TableCell::new("Адаптація програмного забезпечення під зміни у інфрастуктури та компоненти інформаціних систем. Підвищення ефективності \
надання віддаленої памʼяті за рахунок використання покращень інших продуктів та технологій (наприклад, нові можливості апартної платформи які можна використати \
для більш ефективного переміщення даних між вузлами).".into()).width(competition_table_description_width),
                ],
                vec![
                    TableCell::new("4".into()).width(competition_table_number_width),
                    TableCell::new("Конкуренція за видами товарів: товарно-родова".into()).width(competition_table_property_width),
                    TableCell::new("Програмні продукти що конкурують спираються на різні підходи для надання віддаленої памʼяті. Конкурентну перевагу надає \
не більш ефективна реалізація підходу, а використання більш ефеткивних підходів.".into()).width(competition_table_description_width),
                    TableCell::new("Постійний аналіз впливу різних факторів на рівень швидкодії віддаленої памʼяті, внесення значних змін у методи її надання \
якщо це необхідно для покращення швидкодії.".into()).width(competition_table_description_width),
                ],
                vec![
                    TableCell::new("5".into()).width(competition_table_number_width),
                    TableCell::new("За характером конкурентних переваг: нецінова".into()).width(competition_table_property_width),
                    TableCell::new("Найбільшим фактором є кількість оперативної памʼяті що було перерозподілено для більш ефективного використання у \
інформаційній системі у порівнянні з затратами на інтеграцію віддаленої памʼяті у програмне забезпечення.".into()).width(competition_table_description_width),
                    TableCell::new("Постійний моніторинг ефективності віддаленої памʼяті для різних типів програмного забзепечення, пошук шляхів зменшення \
затримки доступу до даних та збільшення відношення обсягу даних у віддаленій памʼяті до обсягу даних у локальній памʼяті".into()).width(competition_table_description_width),
                ],
                vec![
                    TableCell::new("6".into()).width(competition_table_number_width),
                    TableCell::new("За інтенсивністю: марочна".into()).width(competition_table_property_width),
                    TableCell::new("Створення бренду програмного продукту що підвищує ефективність використання ресурсів у центрі обробки даних".into()).width(competition_table_description_width),
                    TableCell::new("Робота над розвитком бренду".into()).width(competition_table_description_width),
                ],
            ],
            "Ступеневий аналіз конкуренції на ринку".to_owned()
        ).with_split(vec![3])),

        paragraph("Для більш детального аналізу умов конкуренції в галузі використовується модель М. Портера. Детальний аналіз умов конкуренції на ринку \
наведено у таблиці 5.9."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Складові аналізу".into()).columns(2),
                TableCell::new("Висновки".into()),
            ],
            vec![
                vec![
                    TableCell::new("Прямі конкуренти в галузі".into()),
                    TableCell::new("Відсутні".into()),
                    TableCell::new("В даний момент відсутні інші програмні продукти для надання віддаленої памʼяті що є доступними для практичного \
використання.".into()),
                ],
                vec![
                    TableCell::new("Потенційні конкуренти в галузі".into()),
                    TableCell::new("Хмарна інфраструктура".into()),
                    TableCell::new("Альтернативою збільшення ефективності використання серверного обладнання є використання хмарної інфраструктури, де \
інформаційній системі надається рівно стільки ресурсів скільки є потрібним для її ресурсів. Слід аналізувати типові випадки клієнтів для демонстрації, що \
збільшення ефективності використання серверного обладнання за допомогою віддаленої памʼяті є більш економічно вигідним у порівнянні з використанням \
хмарної інфраструктури.".into()),
                ],
                vec![
                    TableCell::new("Постачальники".into()),
                    TableCell::new("Постачальники серверного обладнання".into()),
                    TableCell::new("Незважаючи на те, що цей програмний продукт не залежить від спеціалізованого апаратного забезпечення, особливості \
апаратної платформи (серверного обладнання) мають значний вплив на ефективність роботи віддаленої памʼяті.".into()),
                ],
                vec![
                    TableCell::new("Клієнти".into()),
                    TableCell::new("Оператори великих центрів обробки даних".into()),
                    TableCell::new("Важливим є те, що оператори великих центрів обробки даних зазвичай мають можливість розробити власний програмний продукт \
для надання віддаленої памʼяті. Це означає, що програмний продукт що розробляється у цьому проєкті повинен мати властивості що роблять його використання \
більш привабливим за розробку власного рішення окремими операторами центрів обробки даних.".into()),
                ],
                vec![
                    TableCell::new("Товари-замінники".into()),
                    TableCell::new("Програмні продукти що виконують функції віртуалізації або що є компонентами хмарної інфраструктури.".into()),
                    TableCell::new("Програмні продукти такого типу можуть підвищувати ефективність використання оперативної памʼяті іншими методами, \
наприклад за допомогою зміни механізмів керування памʼяттю на рівні операційної системи чи віртуальної машини. Однак, такий підхід є менш ефективним у \
порівнянні з інтеграцією віддаленої памʼятті на рівні програмного забезпечення, що зумовлено меншою кількістю доступної інформації про доступ до даних \
у віддаленій памʼяті.".into()),
                ],
            ],
            "Аналіз конкуренції в галузі за М. Портером".to_owned(),
        ).with_split(vec![5])),
        paragraph("З проведеного аналізу можна зробити висновок що з огляду на конкурентну ситуацію існуює можливість роботи на ринку. Для того, щоб \
проєкт був конкурентноспроможним на ринку він повинен мати більшу ефективність у порівнянні з іншими методами збільшення ефективності використання \
оперативної памʼяті."),

        paragraph(vec![
            "Фактори конкурентноспроможності докладніше розглянуто в таблиці 5.10.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Фактор конкурентноспроможності".into()),
                TableCell::new("Обґрунтування".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("До і після продажне обслуговування".into()),
                    TableCell::new("Надання консультацій перед інтеграцією віддаленої памʼяті у інформаційну систему надає можливість клієнту використовувати \
віддалену памʼять більш ефективно у специфічних для клієнта умовах (ціль: правильно обрати дані що будуть зберігатися у віддаленій памʼяті, та компоненти \
програмного забезпечення що будуть з ними працювати, правильно обрати параметри налаштування віддаленої памʼяті для більш високої ефективності). Консультації \
після інтеграції віддаленої памʼяті дає можливості змінювати налаштування для більш ефективної роботи та встановити напрямки розвитку технології. Все це \
приводить до більш широкого впровадження програмного продукту.".into()),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Модифікація по замовленню".into()),
                    TableCell::new("Модифікація програмного продукту для надання віддаленої памʼяті по замовленню від клієнта дає можливість адаптувати \
програмний продукт для більш ефективної роботи з програмною та апаратною платформою клієнта. Це дозволяє отримати перевагу над продукцією конкурентів завдяки \
більшій ефективності.".into()),
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("Легкість розгортання та інтеграції".into()),
                    TableCell::new("Розробка простих методів розгоратння віддаленої памʼяті та її інтеграції у програмне забезпечення робить програмний продукт \
більш конкурентноспроможнім за програмні продукти конкурентів що мають більш складне розгортання та налаштування чи вимоги до програмної та апартної платформи.".into()),
                ],
                vec![
                    TableCell::new("4".into()),
                    TableCell::new("Рівень швидкодії".into()),
                    TableCell::new("Більш високий рівень швидкодії (більш низька затримка доступу до даних у віддаленій памʼяті) дає можливість використовувати \
віддалену памʼять у програмному забезпеченні що є більш чутливим до додаткових затримок. Це дає перевагу над програмним забезпеченням для надання віддаленої \
памʼяті що має більш низьку ефективність, що обмежує область застосування.".into()),
                ],
            ],
            "Обґрунтування факторів конкуретноспроможності".to_owned(),
        ).with_split(vec![4])),

        paragraph("Базуючись на визначених факторах конкурентноспроможності було проведено аналіз слабких сторін стартап-проекту. Порівняльний аналіз \
наведено у таблиці 5.11."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()).merge_continue(),
                TableCell::new("Фактор конкурентноспроможності".into()).merge_continue(),
                TableCell::new("Бали 1-20".into()).merge_continue(),
                TableCell::new("Рейтинг товарів-конкурентів у порівнянні з запропонованим".into()).columns(7),
            ],
            vec![
                vec![
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new("".into()).merge_continue(),
                    TableCell::new(TextSpan::Bold(Box::new("-3".into()))),
                    TableCell::new(TextSpan::Bold(Box::new("-2".into()))),
                    TableCell::new(TextSpan::Bold(Box::new("-1".into()))),
                    TableCell::new(TextSpan::Bold(Box::new("0".into()))),
                    TableCell::new(TextSpan::Bold(Box::new("+1".into()))),
                    TableCell::new(TextSpan::Bold(Box::new("+2".into()))),
                    TableCell::new(TextSpan::Bold(Box::new("+3".into()))),
                ],
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("До і після продажне обслуговування".into()),
                    TableCell::new("16".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("+".into()), // +1
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Модифікація по замовленню".into()),
                    TableCell::new("18".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("+".into()), // +2
                    TableCell::new("".into()),
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("Легкість розгортання та інтеграції".into()),
                    TableCell::new("19".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("+".into()), // +3
                ],
                vec![
                    TableCell::new("4".into()),
                    TableCell::new("Рівень швидкодії".into()),
                    TableCell::new("15".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                    TableCell::new("+".into()), // +1
                    TableCell::new("".into()),
                    TableCell::new("".into()),
                ],
            ],
            "Порівняльний аналіз сильних та слабких сторін програмного забезпечення для надання віддаленої памʼяті у розподілених системах".to_owned(),
        )),

        paragraph(vec![
            "Фінальним етапом ринкового аналізу можливостей впровадження програмного продукту для надання віддаленої памʼяті є проведення SWOT-аналізу. \
Сильні та слабкі сторони проекти, а також можливості та загрози наведені у таблиці 5.12.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new(TextSpan::Bold(Box::new("Сильні сторони (S):".into()))),
                TableCell::new(TextSpan::Bold(Box::new("Слабкі сторони (W):".into()))),
            ],
            vec![
                vec![
                    TableCell::new(vec![
                        "- зниження витрат на серверне обладнання, оскільки його ресурси використовуються більш ефективно;".into(),
                        TextSpan::Break,
                        "- можливість обробляти набори даних розмір яких є більшим за обсяг локальної памʼяті, без значних змін у програмний код.".into(),
                    ].into()),
                    TableCell::new(vec![
                        "- негативний ефект на швидкодію програмного забезпечення, що обмежує область застосування (типи програмного забезпечення для якого \
використання віддаленої памʼяті є допустимим);".into(),
                        TextSpan::Break,
                        "- потенційна наявність альтернатив у вигляді відпоідного функціоналу у програмному забезпеченні віртуалізації та хмарної \
інфраструктури.".into(),
                    ].into()),
                ],
                vec![
                    TableCell::new(TextSpan::Bold(Box::new("Можливості (O):".into()))),
                    TableCell::new(TextSpan::Bold(Box::new("Загрози (T):".into()))),
                ],
                vec![
                    TableCell::new(vec![
                        "- інтеграція у популярне програмне забезпечення з відкритим кодом що широко використовується (наприклад, бази даних);".into(),
                        TextSpan::Break,
                        "- зміни на ринку віртуалізації, хмарних обчислень, зміна підходів у архітектурі типового програмного забезпечення інформаційних \
систем що можуть створити привід для інтеграції віддаленої памʼяті у ці системи.".into(),
                    ].into()),
                    TableCell::new(vec![
                        "- незацікавленність клієнтів використанням віддаленої памʼяті на користь інших методів підвищення ефективності використання \
ресурсів або зниження витрат на серверне обладнання;".into(),
                        TextSpan::Break,
                        "- поява альтернатив що дозволяють іншими методами знизити використання оперативної памʼяті.".into(),
                    ].into()),
                ],
            ],
            "SWOT-аналіз стартап-проекту".to_owned(),
        )),
        paragraph("В результаті проведеного SWOT-аналізу можна зробити висновок про необхідність роботи над легкістю інтеграції та розгортання віддаленої \
памʼяті, покращення рівня швидкодії для того щоб робити її використання більш виправданим, робити інтеграцію у існуюче програмне забезпечення що широко \
використовується різними клієнтами. Це дозволить зробити потенційних користувачів більш зацікавленими у використанні віддаленої памʼяті, знизити рівень \
витрат необхідних щоб почати роботу з цим програмним продуктом, розширити область застосування."),

        paragraph(vec![
            "У таблиці 5.13 наведено альтернативи ринкового впровадження стартап-проекту.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Альтернатива (орієнтовний комплекс заходів) ринкової поведінки".into()),
                TableCell::new("Ймовірність отримання ресурсів".into()),
                TableCell::new("Строки реалізації".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Інтеграція віддаленої памʼяті у демонстраційне програмне забезпечення (популярне програмне забезпечення з відкритим \
програмним кодом), дослідження ефективності у кожному випадку, створення звітів".into()),
                    TableCell::new("Час".into()),
                    TableCell::new("3 місяці".into()),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Статті, відео-огляди, сторінки в соціальних мережах".into()),
                    TableCell::new("Час та фінансові ресурси".into()),
                    TableCell::new("6 місяців".into()),
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("Демонстрація програмного продукту на тематичних конференціях та заходах".into()),
                    TableCell::new("Власні кошти".into()),
                    TableCell::new("місяць".into()),
                ],
            ],
            "Альтернативи ринкового впровадження стартап-проекту".to_owned(),
        )),
        paragraph("Обраною альтернативою обрано інтеграцію віддаленої памʼяті у демонстраційне програмне забезпечення, оскільки отримання ресурсів для \
цього є найбільш йомвірним, строки відносно стислі, а результат у вигляді звітів по зниження використання ресурсів для демонстраційного програмного \
забезпечення є ефективним для підвищення зацікавленності серед потенційних користувачів."),

        subsection_header("Розроблення ринкової стратегії проекту"),
        paragraph(vec![
            "Першим кроком розроблення ринкової стратегії проекту є визначення стратегії охоплення ринку. Опис цільових груп потенційних користувачів \
наведено у таблиці 5.14.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Опис профілю цільової групи потенційних клієнтів".into()),
                TableCell::new("Готовність споживачів сприйняти продукт".into()),
                TableCell::new("Орієнтовний попит в межах цільової групи (сегменту)".into()),
                TableCell::new("Інтенсивність конкуренції в сегменті".into()),
                TableCell::new("Простота входу у сегмент".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Індивідуальні розробники програмного забезпечення що працюють над невеликими проектами".into()),
                    TableCell::new("Низька, оскільки підвищення ефективності використання ресурсів не буде мати значного результата через невелику кількість \
обладнання що використовується для розгортання інформаційної системи.".into()),
                    TableCell::new("1%".into()),
                    TableCell::new("Низька".into()),
                    TableCell::new("Середня, оскільки ця категорія користувачів зацікавлена в ознайомленні з віддаленою памʼяттю як альтернативою або \
доповненням інших методів підвищення ефективності використання ресурсів, але при цьому зазвичай не мають середовища де використання віддаленої памʼяті \
було б виправданим.".into()),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Підприємства середнього розміру що є операторами інформаційних систем".into()),
                    TableCell::new("Середня, оскільки такі підприємства зацікавлені у зниженні затрат на серверне обладнання, при цьому можуть мати обмежені \
ресурси для впровадження віддаленої памʼяті.".into()),
                    TableCell::new("60%".into()),
                    TableCell::new("Низька".into()),
                    TableCell::new("Легка, оскільки в більшості випадків для цієї категорії єдиною умовою є вартість впровадження що є меншою за вартість \
ресурсів що було використано більш ефективно.".into()),
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("Великі підприємства що є операторами великих інформаційних систем та центрів обробки даних".into()),
                    TableCell::new("Висока, оскільки такі підприємства зацікавлені в збільшенні ефективності використання ресурсів що має прямий вплив на \
одну з головних їх категорії витрат.".into()),
                    TableCell::new("90%".into()),
                    TableCell::new("Середня, зумовлена тим що таке підприємство має ресурси та зацікавленість у розробці власного рішення".into()),
                    TableCell::new("Середня, через специфічні у кожному випадку вимоги, складністю інтеграції через складність програмного забезпечення \
інформаційної системи, конкуренцією з власними розробками підприємства.".into()),
                ],
                vec![
                    TableCell::new("4".into()),
                    TableCell::new("Оператори публічних хмарних платформ".into()),
                    TableCell::new("Середня, оскільки такі підприємства не зважди мають можливість змінювати програмний код програмного забезпечення що \
працює у їх центрах обробки даних (така можливість є лише для високорівневого програмного забезпечення що вони самі розробляють та надають как послугу). \
Незважаючи на це, зацікавлені в збільшенні ефективності використання ресурсів оскільки це одна з їх конкурентних переваг над іншими операторами хмарних \
платформ.".into()),
                    TableCell::new("90%".into()),
                    TableCell::new("Середня, зумовлена тим що таке підприємство має ресурси та зацікавленість у розробці власного рішення".into()),
                    TableCell::new("Висока складність входу через високі вимоги до програмного продукту що надає віддалену памʼять та значної зацікавленості \
у розробці власного рішення.".into()),
                ],
                vec![
                    TableCell::new(TextSpan::Italic(Box::new("Які цільові групи обрано: підприємства середнього (головний фокус) та великого розміру.".into()))).columns(6),
                ],
            ],
            "Вибір цільових груп потенційних користувачів".to_owned(),
        ).with_split(vec![2, 2])),

        paragraph(vec![
            "На основі проведеного аналізу цільових груп потенційних користувачів визначається стратегія охоплення ринку. Базову стратегію розвитку \
сформовано у таблиці 5.15.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Обрана альтернатива розвитку проекту".into()),
                TableCell::new("Стратегія охоплення ринку".into()),
                TableCell::new("Ключові конкурентноспроможні позиції відповідно до обраної альтернативи".into()),
                TableCell::new("Базова стратегія розвитку".into()),
            ],
            vec![
                vec![
                    TableCell::new("Надання функціональності що відсутня у товарів-замінників".into()),
                    TableCell::new("Інтеграція віддаленої памʼяті у популярне програмне забезпечення з відкритим програмним кодом, дослідження ефективності, \
повідомлення про результати та рекламувати в соціальних мережах, на конференціях і т.д.".into()),
                    TableCell::new("Легкість інтеграції та наявність підтвержень високої ефективності програмного продукту та можливості застосування \
у різних сценаріях, на відміну від конкурентів.".into()),
                    TableCell::new("Стратегія диференціації".into()),
                ],
            ],
            "Визначення базової стратегії розвитку".to_owned(),
        )),

        paragraph(vec![
            "Наступним важливим кроком є вибір стратегії конкурентної поведінки. Визначення базової стратегії конкурентної поведінки наведено у \
таблиці 5.16.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Чи є проект «першопрохідцем» на ринку?".into()),
                TableCell::new("Чи буде копанія шукати нових споживачів, або забирати існуючих у конкурентів?".into()),
                TableCell::new("Чи буде компанія копіювати основні характеристики товару конкурента, і які?".into()),
                TableCell::new("Стратегія конкурентної поведінки".into()),
            ],
            vec![
                vec![
                    TableCell::new("Так, тому що існуючи програмні продукти не є доступними для використання або мають характеристики що заважають їх \
використанню на практиці".into()),
                    TableCell::new("Компанія буде шукати як нових користувачів зацікавлених у підвищенні ефективності використання ресурсів, так і \
переконувати користувачів конкурентів виконати міграцію на програмний продукт компанії".into()),
                    TableCell::new("Так, копіювання особливостей методу інтеграції у програмне забезпечення (інтерфейс клієнтської бібліотеки) є необхідним \
щоб користувачі програмного продукту конкурентів розглядали можливість міграції".into()),
                    TableCell::new("Стратегія лідера".into()),
                ],
            ],
            "Визначення базової стратегії конкурентної поведіники".to_owned(),
        )),
        paragraph("Стратегією конкурентної поведінки було обрано стратегію лідера, оскільки проект є «першопрохідцем» на ринку і головною метою є переконати \
потенційних користувачів використовувати цей програмний продукт для підвищення ефективності використання ресурсів, а не перейти з рішення конкурентів \
що вони вже використовують для цієї задачі."),

        paragraph(vec![
            "Наступним кроком є розробка стратегії позиціонування на основі вимог до продукту та обраної базової стратегії розвитку. Визначення стратегії \
позиціонування наведено у таблиці 5.17.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Вимоги до товару цільової аудиторії".into()),
                TableCell::new("Базова стратегія розвитку".into()),
                TableCell::new("Ключові конкуретноспроможні позиції".into()),
                TableCell::new("Вибір асоціацій, які мають сформувати комплексну позицію власного проекту".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Легкість у розгортанні".into()),
                    TableCell::new("Стратегія диференціації".into()),
                    TableCell::new("Технологічні властивості програмного продукту що знижують витрати на розгортання віддаленої памʼяті".into()),
                    TableCell::new("Розгортання за допомогою однієї команди чи кліку".into()),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("Легкість у інтеграції у програмне забезпечення".into()),
                    TableCell::new("Стратегія диференціації".into()),
                    TableCell::new("Клієнт віддаленої памʼяті що у зручним у використанні у різних типах програмного забезпечення".into()),
                    TableCell::new("Працювати з даними у віддаленій паʼмяті настільки ж легко, як з даними у локальній памʼяті".into()),
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("Рівень відмовостійкості що є допустимим для інформаційної системи, в яку інтегрується віддалена памʼять".into()),
                    TableCell::new("Стратегія спеціалізації".into()),
                    TableCell::new("Технологічні властивості програмного продукту що дозоляють відновлювати дані у випадку відмови вузлів розподіленої \
системи".into()),
                    TableCell::new("Розміщення даних на декількох вузлах".into()),
                ],
                vec![
                    TableCell::new("4".into()),
                    TableCell::new("Рівень швидкодії що дозволяє інформаційній системі, в яку інтегрується віддалена памʼять, дотримуватись цільового \
рівня обслуговування (SLO)".into()),
                    TableCell::new("Стратегія диференціації".into()),
                    TableCell::new("Технологічні властивості програмного продукту що дозволяють мати низький рівень затримки доступу до даних у віддаленій \
памʼяті".into()),
                    TableCell::new("Інформаційна система зберігає прийнятний рівень швидкодії після інтеграції віддаленої памʼяті".into()),
                ],
            ],
            "Визначення стратегії позиціонування".to_owned(),
        ).with_split(vec![4])),
        paragraph("Отже, основою ринкової стратегії буде залучення уваги підприємств середнього та великого розміру що є зацікавленими у збільшенні \
ефективності використання ресурсів центрів обробки даних. Стратегією позиціонування обрано стратегію диференціації для задоволення вимог цільової аудиторії \
шляхом розвитку технологічних властивостей програмного продукту. Оскільки програмний продукт є «першопрохідцем» на ринку, то стратегією конкурентної \
поведінки обрано стратегію лідера."),

        subsection_header("Розроблення маркетингової програми стартап-проекту"),
        paragraph(vec![
            "Першим кроком розроблення маркетингової програми стартап-проекту є формування маркетингової концепції товару. У таблиці 5.18 \
визначено ключові переваги концепції потенційного товару.".into(),
            TextSpan::PageBreak,
        ]),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Потреба".into()),
                TableCell::new("Вигода, яку пропонує товар".into()),
                TableCell::new("Ключові переваги перед конкурентами (існуючі або такі, що потрібно створити)".into()),
            ],
            vec![
                vec![
                    TableCell::new("Збільшення ефективності використання ресурсів центру обробки даних".into()),
                    TableCell::new("Зниження витрат на додаткові сервери або розширення оперативної памʼяті в існуючому обладнанні за рахунок \
перенесення частин даних з памʼяті більш завантажених серверів у памʼять менш завантажених (тобто, за рахунок використання віддаленої памʼяті)".into()),
                    TableCell::new("Більш легке розгортання, більш легка інтеграція у програмне забезпечення інформаційної системи, \
більш високий рівень швидкодії (нижча затримка доступу до даних у віддаленій памʼяті).".into()),
                ],
            ],
            "Визначення ключових переваг концепції потенційного товару".to_owned(),
        )),

        paragraph("Наступним кроком є розробка трирівневої маркетингової моделі товару. Опис трьох рівнів моделі товару наведено у таблиці 5.19."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Рівні товару".into()).width(product_levels_table_property_width),
                TableCell::new("Сутність та складові".into()).columns(3),
            ],
            vec![
                vec![
                    TableCell::new("1. Товар за задумом".into()).width(product_levels_table_property_width),
                    TableCell::new("Збільшення ефективності використання ресурсів центру обробки даних, а саме більш ефективне використання \
оперативної памʼяті у розподіленій системі".into()).columns(3),
                ],
                vec![
                    TableCell::new("2. Товар у реальному виконанні".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Властивості/характеристики".into()),
                    TableCell::new("М/Нм".into()),
                    TableCell::new("Вр/Тх/Тл/Е/Ор".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Розгортання".into()),
                    TableCell::new("Нм".into()),
                    TableCell::new("Вр/Тх/Е".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Інтеграція у програмне забезпечення".into()),
                    TableCell::new("М".into()),
                    TableCell::new("Вр/Тх/Тл/Е".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Забезпечення відмовостійкості".into()),
                    TableCell::new("Нм".into()),
                    TableCell::new("Вр/Тх".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Забезпечення високого рівня швидкодії".into()),
                    TableCell::new("М".into()),
                    TableCell::new("Вр/Тх/Тл".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Інтерфейс для моніторингу та керування".into()),
                    TableCell::new("Нм".into()),
                    TableCell::new("Вр/Тх/Е".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Адаптація до запитів клієнта".into()),
                    TableCell::new("М".into()),
                    TableCell::new("Вр/Тх/Ор".into()),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Якість: покриття усіх компонентів автоматизованими тестами".into()).columns(3),
                ],
                vec![
                    TableCell::new("2. Товар у реальному виконанні".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Пакування: компоненти віддаленої памʼяті надаються у вигляді Docker контейнерів, конфігурації \
Kubernetes на Helm для більш простого розгортання".into()).columns(3),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Марка: назва організації-розробника та назва програмного продукту".into()).columns(3),
                ],
                vec![
                    TableCell::new("3. Товар із підкріпленням".into()).merge_restart().width(product_levels_table_property_width),
                    TableCell::new("До продажу: наявність документації для усіх компонентів програмного продукту для надання віддаленої \
памʼяті, надання послуг консультації по розгортанню та інтеграції віддаленої памʼяті".into()).columns(3),
                ],
                vec![
                    TableCell::new("".into()).merge_continue().width(product_levels_table_property_width),
                    TableCell::new("Після продажу: моніторинг стану віддаленої памʼяті, надання послуг консультацій по обслуговуванню та \
налаштуванню, випуск оновлень".into()).columns(3),
                ],
                vec![
                    // i hate this and do believe that there is nothing bad in such software being copied. But still, it is required from
                    // me to write something here.
                    TableCell::new("За рахунок чого потенційний товар буде захищено від копіювання: запатентовані алгоритми заміщення \
проміжків що є фактором швидкодії віддаленої памʼяті, послуги консультації базуються на знаннях та досвіді яких немає у конкурентів".into()).columns(4),
                ]
            ],
            "Опис трьох рівнів моделі товару".to_owned(),
        ).with_split(vec![10])),
        paragraph("М/Нм – монотонні/немонотонні; Вр/Тх/Тл/Е/Ор – вартісні/ технічні/ технологічні/ ергономічні/ органолептичні."),

        paragraph("Далі слід визначити цінові межі, якими необхідно керуватись при встановленні ціни на потенційний товар. Визначення \
меж встановлення ціни наведено у таблиці 5.20."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Рівень цін на товари-замінники".into()).font_size(12),
                TableCell::new("Рівень цін на товари-аналоги".into()).font_size(12),
                TableCell::new("Рівень доходів цільової групи споживачів".into()).font_size(12),
                TableCell::new("Верхня та нижня межі встановлення ціни на товар/послугу".into()).font_size(12),
            ],
            vec![
                vec![
                    TableCell::new("Програмне забезпечення віртуалізації для центрів обробки даних: 0-$10тис за кожен процесор (одноразова \
витрата)".into()).font_size(12),
                    TableCell::new("На момент проведення аналізу комерційних товарів-аналогів не існує на ринку".into()).font_size(12),
                    TableCell::new("Сложно визначити через наявність великої кількості різних підприємств у цільовій групі користувачів. \
Але, можна орієнтвуватися на рівнь витрат на підтримку центрів обробки даних такими підприємствами: $500тис-$50млн на рік".into()).font_size(12),
                    TableCell::new("Ліцензія на використання: $0-$1000 (одноразово); Консультації та обслуговування: $100-$20тис (щомісячно)".into()).font_size(12),
                ],
            ],
            "Визначення меж встановлення ціни".to_owned(),
        )),

        paragraph("Наступним кроком є визначення оптимальної системи збуту. У таблиці 5.21 наведено формування системи збуту."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Специфіка закупівельної поведінки цільових клієнтів".into()),
                TableCell::new("Функції збуту, які має виконувати постачальник товару".into()),
                TableCell::new("Глибина каналу збуту".into()),
                TableCell::new("Оптимальна система збуту".into()),
            ],
            vec![
                vec![
                    TableCell::new("За допомогою купленого програмного продукту збільшити ефективність використання ресурсів \
на суму більше ніж вартість програмного забезпечення для надання віддаленої памʼяті".into()),
                    TableCell::new("Постійний доступ до виконуваних файлів компонентів віддаленої памʼяті, документації та інших ресурсів".into()),
                    TableCell::new("Однорівневний або нульовий".into()),
                    TableCell::new("Власноруч, посередники тільки у вигляді оператора приватної хмарної платформи на стороні клієнта".into()),
                ],
            ],
            "Формування системи збуту".to_owned(),
        )),

        paragraph("Останньою складовою маркетингової програми є розроблення концепції маркетингових комунікацій. Концепцію маркетингових \
комунікацій наведено у таблиці 5.22."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("Специфіка поведінки цільових клієнтів".into()),
                TableCell::new("Канали комунікацій, якими користуються цільові клієнти".into()),
                TableCell::new("Ключові позиції, обрані для позиціонування".into()),
                TableCell::new("Завдання рекламного повідомлення".into()),
                TableCell::new("Концепція рекламного звернення".into()),
            ],
            vec![
                vec![
                    TableCell::new("Цільові клієнти зацікавлені в зниженні витрат на підтримку центру обробки даних (збільшення \
ефективності використання ресурсів) шляхом використання програмних продуктів та внесення змін в інформаційну систему вартість яких є \
нижчою за вартість ресурсів що було використано більш ефективно".into()),
                    TableCell::new("Веб-сайти в інтернеті, соціальні мережі, тематичні конференції".into()),
                    TableCell::new("Легкість у інтеграції та використанні, значний позитивний вплив на ефективність використання \
оперативної памʼяті серверів у центрі обробки даних".into()),
                    TableCell::new("Звернути увагу на використанні віддаленої памʼяті як інструменту зниження витрат на обслуговування \
та розширення центру обробки даних.".into()),
                    TableCell::new("".into()), // TODO
                ],
            ],
            "Концепція маркетингових комунікацій".to_owned(),
        )),

        // TODO: finish this subsection

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
