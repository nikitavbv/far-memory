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

        paragraph("Для більш ефективного розвитку програмного продукту на ринку необхідно провести аналіз рис конкуренції на ринку. Ступеневий аналіз \
конкуренції на ринку розглянуто у таблиці 5.8."),
        Block::Table(TableBlock::new(
            vec![
                TableCell::new("№".into()),
                TableCell::new("Особливості конкурентного середовища".into()),
                TableCell::new("В чому проявляється дана характеритстика".into()),
                TableCell::new("Вплив на діяльність підприємства (можливі дії компанії, щоб бути конкурентноспроможною)".into()),
            ],
            vec![
                vec![
                    TableCell::new("1".into()),
                    TableCell::new("Тип конкуренції: чиста".into()),
                    TableCell::new("Програмний продукт для забезпечення віддаленої памʼяті не може повністю замінити інше програмне забезпечення для \
надання віддаленої памʼяті, можуть співіснувати навіть у межах одного центру обробки даних; Ринок має умови для входу і виходу; Ціна відрізняється між \
конкурентами.".into()),
                    TableCell::new("Розробка програмного продукту з властивостями, яких немає у конкурентів. Встановлення ціни що відображає рівень переваг \
над інишими програмними продуктами.".into()),
                ],
                vec![
                    TableCell::new("2".into()),
                    TableCell::new("За рівнем конкурентної боротьби: мультинаціональний бізнес".into()),
                    TableCell::new("Програмний продукт не потребує значних змін для використання у закордонних центрах обробки даних.".into()),
                    TableCell::new("Використання великої кількості програмного забезпечення для інтеграції з усього світу. Орієнтація на інтеграцію у \
найбільші світові інформаційні системи та центри обробки даних та використання цього для подальшого розвитку продукту.".into()),
                ],
                vec![
                    TableCell::new("3".into()),
                    TableCell::new("За галузевою ознакою: міжгалузева".into()),
                    TableCell::new("Конкуренція з іншими типами рішень: на рівні інфраструктури чи апаратних компонентів які також могли б вирішувати \
проблему більш ефективного використання оперативної памʼяті".into()),
                    TableCell::new("Адаптація програмного забезпечення під зміни у інфрастуктури та компоненти інформаціних систем. Підвищення ефективності \
надання віддаленої памʼяті за рахунок використання покращень інших продуктів та технологій (наприклад, нові можливості апартної платформи які можна використати \
для більш ефективного переміщення даних між вузлами).".into()),
                ],
                vec![
                    TableCell::new("4".into()),
                    TableCell::new("Конкуренція за видами товарів: товарно-родова".into()),
                    TableCell::new("Програмні продукти що конкурують спираються на різні підходи для надання віддаленої памʼяті. Конкурентну перевагу надає \
не більш ефективна реалізація підходу, а використання більш ефеткивних підходів.".into()),
                    TableCell::new("Постійний аналіз впливу різних факторів на рівень швидкодії віддаленої памʼяті, внесення значних змін у методи її надання \
якщо це необхідно для покращення швидкодії.".into()),
                ],
                // TODO: add two more rows.
            ],
            "Ступеневий аналіз конкуренції на ринку".to_owned()
        )),

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
