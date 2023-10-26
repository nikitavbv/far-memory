use crate::thesis::engine::{Block, subsection_header, section_header, paragraph, ImageBlock, TextSpan};

pub fn documentation() -> Block {
    Block::Multiple(vec![
        section_header("far memory"),
        Block::Note(r#"Please note that most parts of documentation for this project are in Ukrainian because I am working on this in scope of my thesis at Kyiv Polytechnic Institute and I
need to be able to refer to this documentation when talking to thesis supervisors and other people from the university. I will probably add English translation later."#.to_owned()),

        section_header("Віддалена памʼять"),
        paragraph(
          "Віддалена памʼять - клас памʼяті, дані якого зберігаються на пристроях з часом доступу більшим ніж у оперативної памʼяті (диски, віддалені вузли - у цій роботі розглядається останнє) та
          методи які забезечують доступ до цих даних з рівнем затримки, відмовостійкістю та легкістю використання прийнятним для використання у прикладному програмному забезпеченні."
        ),
        paragraph("Ціллю віддаленої памʼяті є підвищення рівню використання оперативної памʼяті у датацентрах (у сучасному датацентрі рівень використання RAM становить близько 60%). Більш високий рівень використання
        памʼяті забезпечується наданням доступу до памʼяті вузлів з вільною памʼяттю вузлам які її можуть використати."),
        paragraph("Віддалена памʼять є актуальною навіть при віртуалізації та надмірній підписці (oversubscription) ресурсів, тому що вільна памʼять залишається навіть при дуже ефективному планувальнику задач. Крім цього,
        використання віддаленої памʼяті дозволяє працювати з обсягом даних що перевищує фізичний обʼєм памʼяті на вузлі без значних змін у код програмного забезпечення."),

        section_header("Формалізація задачі"),
        paragraph("Для прикладного програмного забезпечення, в яке інтегрована віддалена памʼять, максимізувати частку даних що зберігається у віддаленій памʼяті при умові дотримання вимог швидкодії."),
        paragraph("Наприклад, цікаво розглянути популярне в даний момент часу програмне забезпечення для генерації тексту за допомогою великих мовних моделей (LLM).
Інтеграція віддаленої памʼяті у цьому випадку є привабливою через його особливості: необхідність тримати у опретивній памʼяті великий обʼєм даних (тільки ваги нейронної
мережі займають десятки гігабайт), не до усіх даних потрібен доступ одночасно, але у запитах до памʼяті є певні закономірності, під які можна оптимізувати параметри
работи віддаленої памʼяті. Якщо взяти просту реалізацію Llama2-7B, то час обробки запиту (генерації токену тексту) становить 4.95c у середньому, при використанні приблизно
26Гб оперативної памʼяті. При умові що час обробки запиту не повинен погіршитись більше ніж на 10%, то задача віддаленої памʼяті у цьому випадку - мінімізувати
використання оперативної памʼяті на вузлі, при цьому час обробки запиту не повинен перевищити 5.45c."),
        paragraph("Крім цього, можна розглянути програмне забезпечення що виконує аналітику на великому обʼємі даних (наприклад, у AIFM для оцінки роботи використовується
аналіз даних поїздок таксі на датасеті розміром ~16Гб). У цьому випадку, віддалена памʼять дозволяє обробляти більше даних ніж обʼєм оперативної памʼяті на одному вузлі.
У доступі до даних зазвичай є закономірності, які можна використовувати, а вимоги до швидкодії для такого програмного забезпечення не є жорсткими."),

        section_header("Підзадачі"),
        paragraph("Нижче наведені підзадачі які потрібно виіршити для реалізації віддаленої памʼяті у порядку їх важливості."),

        subsection_header("Зниження затримки доступу (latency)"),
        paragraph("Затримка доступу до памʼяті має прямий вплив на швидкодію програмного забезпечення, тому її потрібно мінімізувати. Час читання даних з оперативної памʼяті нижчий за час читання даних по мережі, тому зниження затримки базується на тому, що потрібні дані вчасно переміщуються з памʼяті віддалених вузлів до оперативної памʼяті."),
        Block::Image(ImageBlock::new("latency.jpg".to_owned(), "затримка доступу до даних у віддаленій памʼяті".to_owned())),
        paragraph("Способи зниження затримки, які можна розглянути для використання:"),
        paragraph(vec![
          TextSpan::Bold("- групування обʼєктів".to_owned()),
          " таким чином, щоб обʼєкти доступ до яких відбувається частіше, знаходились в \"гарячих сторінках (spans)\". Обʼєкти, доступ до яких відбувається рідше, попадають у \"холодні сторінки\". Таким чином, у локальній памʼяті знаходиться більше гарячих обʼєктів і кількість запитів до інших вузлів знижається.".into(),
          r#" Такий підхід використовується у Carbink, де окремий потік переміщує обʼєкти між локальними сторінками для більш ефективного групування."#.into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- запит сторінок наперед".to_owned()),
          r#". Наприклад у AIFM структури даних оптимізвані завчасно завантажувати наступні сторінки. Наприклад, у масиві або списку під час ітерації завантажується наступни сторінки."#.into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- зниження фрагментації".to_owned()),
          ". При більш щільному розміщенні обʼєктів у сторінках, кількість сторінок що потрібно держати у памʼяті знижується, що також позитивно впливає на затримку. У Carbink це вирішується за допомогою використання size classes для обʼєктів, як у TCMalloc. Крім цього, розповсюдженим підходом є compaction, тобто пересення обʼєктів з менш завантажених на більш завантажені сторінки.".into(),
        ]),
        paragraph(vec![
          r#"Існуючі реалізації спираються на прості еврістики: рахування кількості доступів до обʼєктів для їх групування, запит наступної сторінки для структури даних. Розвитком цього може бути використання більш складних моделей для керування групуванням обʼєктів,
переміщення сторінок у віддалену памʼять та з неї, вирішення проблеми фрагментації. Методи які слід розглянути: еврістичні підходи, побудова FSM за зібраною статистикою,
ML моделі (у тому числі RNN) та ін."#.into(),
        ]),
        paragraph(vec![
          "Також привабливим є збір статистики під час роботи програмного забезпечення та оптимізація моделей у реальному часі на її основі. Зібрана статистика може використовуватись як для побудови моделей, оптимізації їх
          гіперпараметрів під час роботи а також для оцінки якості роботи віддаленої памʼяті. Такий підхід використовується наприклад у \"Software-defined far memory in warehouse-scale computers\", де зібрана статистика
          використовується для оптимізації параметрів zswap (віддалена памʼять у цьому випадку - памʼять на диску, а не на віддалених вузлах).".into(),
        ]),

        subsection_header("Забезпечення відмовостійкості"),
        paragraph(
          r#"Оскільки сторінки памʼяті зберігаються на віддалених вузлах, то віддалені вузли становляться частиною домену збою (failure domain) для програмного забезпечення, у яке інтегрована віддалена памʼять. Для того, щоб
обмежити негативний вплив на надійність програмного забезпечення, можна використовувати наступні методи для сторінок памʼяті у віддаленій памʼяті:"#
        ),
        paragraph(vec![
          TextSpan::Bold("- копія памʼяті на диску".to_owned()),
          ". При відмові віддаленого вузла, відновлення даних відбувається з диску. Недоліком цього підходу є повільне відновлення, та необхідність доступу до диску.".into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- реплікація".to_owned()),
          ". Сторінки памʼяті копіюються на декілька вузлів. При відмові одного з них, дані відновлюються з будь-якого з інших. Недоліком є надмірне використання памʼяті (більше на фактор реплікації).".into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- кодування стиранням".to_owned()),
          " (erasure coding). Наприклад, використовується код Ріда-Соломона для кодування сторінки у 5 частин (3 data shards, 2 parity shards). Ці частини даних розміщуються на різних вузлах, при виході з ладу будь-якого з них, дані можна відновити з інших вузлів. На відміну від реплікації, використовує менше памʼяті для забезпечення надлишковості для відновлення. Кількість частин даних може бути обрана користувачем в залежності від вимог до відмовостікйості.".into(),
        ]),
        Block::Image(ImageBlock::new("shards.jpg".to_owned(), "розміщення частин данних на різних вузлах при використанні erasure coding".to_owned())),
        paragraph("Розміщення частин даних на різних вузлах також дозволяє знизити час доступу до данних, оскільки достатньо отримати дані лише з частини вузлів для відновлення сторінки памʼяті у локальній памʼяті."),

        subsection_header("Інтеграція у існуюче та нове програмне забезпечення"),
        paragraph(
          "Для інтеграції у нове програмне забезпечення (те, де є можливість змінювати реалізацію) доцільним є використання розумних показчиків (з існуючих реалізацій так робить Carbink та AIFM). В межах цієї роботи створюється бібліотека на мові програмування Rust,
          яка надає можливість розробнику обирати які дані будуть зберігатися у віддаленій памʼяті. Бібліотека керує переміщенням даних у та з віддаленої памʼяті автоматично. Створення реалізацій структур даних призначених для
          роботи з віддаленою памʼяттю (як у AIFM) не розглядається, оскільки їх використання можна уникнути, якщо автоматичне завантаження сторінок паʼяті працює достатньо ефективно."
        ),
        Block::Image(ImageBlock::new("integration.png".to_owned(), "використання розумного показчика для розміщення даних у віддаленій памʼяті".to_owned())),
        paragraph(
          "Для інтеграції у існуюче програмне забезпечення, або те, яке написане на інших мовах програмування можна використовувати механізм підкачки (swapping) памʼяті у операційній системі. На відміну від звичайного swap, який
          розміщується на диску, в цьому випадку swap розміщується на віртуальному блоковому пристрої, блоки якого відповідають сторінкам у віддаленій памʼяті. Реалізація блокового пристрою використовує ту ж реалізацію переміщення
          сторінок між локальною та віддаленою памʼяттю, що і для інтеграції на основі розмуних показчиків."
        ),

        section_header("Задача вибору блоків памʼяті для переміщення"),
        paragraph(TextSpan::Link { text: "page replacement algorithm".to_owned(), url: "https://en.wikipedia.org/wiki/Page_replacement_algorithm".to_owned() }),
        paragraph(TextSpan::Link { text: "задача заміщення сторінок".to_owned(), url: "https://uk.wikipedia.org/wiki/%D0%97%D0%B0%D0%B4%D0%B0%D1%87%D0%B0_%D0%B7%D0%B0%D0%BC%D1%96%D1%89%D0%B5%D0%BD%D0%BD%D1%8F_%D1%81%D1%82%D0%BE%D1%80%D1%96%D0%BD%D0%BE%D0%BA".to_owned() }),
        paragraph(
          "Коли під час роботи програмного забезпечення виникає потреба звільнити локальну памʼять через переміщення даних у віддалену (swap out), то системі
потрібно обрати які саме блоки памʼяті будуть переміщені. Ефективним є переміщення тих блоків, доступ до яких не очікується у найближчий час (холодні дані).
Крім цього, система у фоновому режимі обирає які блоки памʼяті перемістити у локальну памʼять (swap in) до того, як вони будуть потрібні (prefetching). "
        ),
        paragraph("Якщо завчасне переміщення блоків памʼяті працює максимально ефективно та потрібні дані завжди потрапляють у локальну памʼять до того, як
доступ до них буде потрібен, то вплив на швидкодію буде мінімальним (так як уникається блокування виконання програмного забезпечення через очікування даних з
віддаленої памяʼті)."),
        paragraph("В залежності від того, як програмне забезпечення працює з памʼяттю, можливість коректно визначати завчасно доступ до блоків памʼяті може
бути обмеженою. Кожен доступ до даних, яких немає у локальній памʼяті уповільнує виконання. Тому ціллю є зниження кількості таких випадків."),
        paragraph("У цій роботі не накладається ніяких обмежень на програмне забезпечення, в яке інтегрується віддалена памʼять. Але чим більше закономірностей
у доступі до даних, тим більш ефективною буде віддалена памʼять. Також для інтеграції віддаленої памʼяті краще підходить програмне забезпечення яке виконує
багато обчислень (інших операцій) на кожен доступ до памʼяті."),

        paragraph(TextSpan::Bold("Вхідні дані:".to_owned())),
        paragraph("\\(S_{n}\\) - блок памʼяті до якого відбувся доступ під час виконання програми (ідентифікується номером)."),
        paragraph("\\(T_{n}\\) - час коли відбувся доступ до блоку памʼяті з моменту запуску програми."),
        paragraph("Вхідні дані містять інформацію про доступ до блоків памʼяті як з поточного, так і з попередніх запусків програми."),
        paragraph(TextSpan::Bold("Вихідні дані:".to_owned())),
        paragraph("\\(P_{n}\\) - коефіцієнти що відповідають ймовірностям доступу до відповідних блоків памʼяті у наступну одиницю часу."),

        Block::Image(ImageBlock::new("model.jpg".to_owned(), "приклад роботи моделі, яка визначає запит на доступ до яких блоків памʼяті може бути отриманий під час подальшого виконання програмного забезпечення".to_owned())),

        paragraph("Блоки з найбільшим значенням коефіцієнту слід першою чергою переміщати у локальну памʼять у фоновому режимі. Блоки з найнижчим - навпаки, переміщувати у віддалену памʼять."),

        paragraph(TextSpan::Bold("Можливі методи вирішення:".to_owned())),
        paragraph("- еврістичний підхід - наприклад, рахувати кількість доступів у деякому вікні часу для визначення гарячих та холодних блоків памʼяті.".to_owned()),
        paragraph("- побудова скінченного автомату (недетрмінованого) - станами можуть бути наприклад запити до блоків памʼяті, переходи між ними - послідовність доступів. Передбачення визначається поточним станом та вагою на ребрах."),
        paragraph("- стохастичні моделі (Марковська модель) - моделювання системи на основі поточного стану та попередньої послідовності подій (доступів до даних)."),
        paragraph("- моделі машинного навчання (згорткові нейронні мережи, трансформери та ін.) - застосування таке ж, як до задачі передбачення послідовності."),

        Block::Image(ImageBlock::new("traces_example.png".to_owned(), "приклад роботи програмного забезпечення, де помітна закономірність у доступі до блоків памʼяті".to_owned())),

        section_header("Існуючі реалізації та їх недоліки"),
        paragraph(vec![
          TextSpan::Bold("- carbink".to_owned()),
          " - найбільш розвинена реалізація. Має закритий код, привʼязана до інфраструктури Google та не є доступною для використання ззовні. Використовує прості
еврістики для оптимізації розташування обʼєктів у памʼяті. Не має методів інтеграції без зміни коду програмного забезпечення.".into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- hydra".to_owned()),
          " - вимагає використання спеціалізованого апаратного забезпечення (rdma).".into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- aifm".to_owned()),
          " - робота з одним віддаленим вузлом, не забезпечує відмовостійкість.".into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- far memory in warehouse scale computers".to_owned()),
          " - використовує zswap для зберігання памʼяті на диску, а не у оперативній памʼяті віддалених вузлів.".into(),
        ]),

        section_header("Результати"),
        paragraph("Результати інтеграції віддаленої памʼяті у демонстраційне програмне забезпечення (llm inference)."),
        Block::Table {
          columns: vec![
            "реалізація".to_owned(),
            "час обробки запиту, c".to_owned(),
            "використання памʼяті, МБ".to_owned(),
          ],
          rows: vec![
            vec!["baseline".to_owned(), "4.95".to_owned(), "27755".to_owned()],
            vec!["far memory for vocab without swapout".to_owned(), "4.9".to_owned(), "27757".to_owned()],
            vec!["far memory for embeddings weights".to_owned(), "5.46".to_owned(), "27258".to_owned()],
            vec!["swap out threshold".to_owned(), "5.3".to_owned(), "27654".to_owned()],
          ],
        },

        section_header("Посилання"),
        paragraph(vec![
          TextSpan::Link {
            text: "The Unsafe Rust Programming Language".to_owned(),
            url: "https://d3m3vilurr.gitbooks.io/the-unsafe-rust-programming-language/content/index.html".to_owned()
          },
        ])
    ])
}
