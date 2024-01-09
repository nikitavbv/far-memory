use crate::thesis::engine::{Block, subsection_header, paragraph, ImageBlock, Reference, reference};

pub fn components() -> Block {
    Block::Multiple(vec![
        subsection_header("Компоненти системи"),
        paragraph("Перший компонент, з якого варто почати розглядати систему це вузли обчислення (compute nodes). Ці вузли є програмним забезпеченням, у яке інтегрується віддалена памʼять. Слід \
зазначити, що вузли розглядаються не з точки зору фізичного обчислювального вузла, а як екземпляр програмного забезпечення, що виконується. Користуватися віддаленою памʼяттю може одночасно різне \
програмне забезпечення, різна кількість його екземплярів, різні версії з різними налаштуваннями. При цьому, як і у звичайному програмному забезпеченні, що працює з локальною оперативною памʼяттю, \
програма що зберігає дані у віддаленій памʼяті не має доступу до даних інших програм."),
        paragraph("Вузел обчислення складається з програмного забезпечення у яке інтегрована віддалена памʼять та клієнта віддаленої памʼяті. Програмне забезпечення працює з клієнтом віддаленої \
памʼяті для розміщення даних у ній та для отримання доступу до цих даних. Крім цього, про властивості програмного забезпечення у яке інтегрується віддалена памʼять не робиться додаткових припущень."),
        paragraph("Основною сутністю, з якою працює клієнт віддаленої памʼяті, є проміжок памʼяті (span). Проміжок памʼяті можна вважати аналогічним сторінці памʼяті у операційній системі, тобто це \
безперервним блоком памʼяті. Проміжок памʼяті має ідентифікатор (64-бітне число), яке ключем у будь-яких операціях повʼязаних з проміжком. Памʼять, яка повʼязана з проміжком, має фіксовану довжину, \
тобто не змінюється після створення проміжку, але одночасно можуть існувати проміжки різної довжини. Дані (послідовність байтів) можуть знаходитись на різних пристроях зберігання, проте вони \
залишаються привʼязаними до цього проміжку."),
        paragraph("Для зберігання даних за межами оперативної памʼяті, клієнт віддаленої памʼяті використовує задану користувачем конфігурацію бекенду (backend). В цій роботі розглядаються різні \
реалізації бекенду, такі як локальна памʼять, SSD диски, памʼять одного чи декількох віддалених вузлів, але головна увага приділяється зберіганню даних у памʼяті багатьох віддалених вузлів. \
Саме переміщення даних у памʼять багатьох віддалених вузлів надає цій розробці практичної цінності."),
        paragraph(vec![
            "Тому вузли зберігання даних (storage nodes) є іншим компонентом системи. Як бекенд віддаленої памʼяті, цей вузел можна розглядати як сховище у форматі ключ-значення, де ключем \
є ідентифікатор проміжку памʼяті, а значенням - дані, що зберігаються у цьому проміжку. Вузли зберігання даних обробляють запити на запис та читання проміжків (при цьому читання видаляє дані, \
так як немає сенсу одночасно зберігати одні й ті самі дані як на вузлах зберігання, так і на вузлах обчислення). Таке формулювання призначення цих вузлів може зробити привабливим використання \
сховищ даних які працюють за схожим принципом, таких як наприклад ".into(),
            reference("Redis", Reference::for_website("Redis", "https://redis.io/")),
            ". Однак, використання такого сховища даних наклало б обмеження на можливості оптимізації протоколу, необхідних для \
мінімізації затримки з урахуванням особливостей задачі, що вирішується.".into(),
        ]),
        paragraph("Наявність багатьох вузлів зберігання та обчислення створює необхідність у компоненті, який керував би роботою кластеру - вузла керування (manager node). Цей компонент \
повинен вирішувати декілька задач. По-перше, за запитом від вузлів обчислення надавати доступ до вузлів зберігання згідно з кількістю памʼяті необхідної для програмного забезпечення. \
Для цього, вузел керування відслідковує рівень завантаженності вузлів у кластері, та пріоритизує ті вузли, де найбільше вільної памʼяті. Крім цього, вузел керування відслідковує стан інших \
компонентів та обробляє випадки виходу з ладу інших вузлів, так і їх запланованого виводу на обслуговування. Також цей компонент надає інструменти для моніторингу стану віддаленої памʼяті. \
У випадку використання локальної памʼяті (як наприклад SSD диску) чи лише одного віддаленого вузла зберігання наявність вузла керування не є необхідним."),
        paragraph("Вузел керування також збирає статистику про роботу системи та визначає параметри її роботи (наприклад, умови за яких проміжки памʼяті переміщуються між локальною та \
віддаленою памʼяттю). Ці обчислення відбуваються на цьому вузлі через наявність більшої інформації про роботу системі ніж та, що є доступною на окремих вузлах. Також, централізоване \
керування параметрами роботи віддаленої памʼяті спрощує її використання для користувачів (розробників програмного забезпечення, у яке вона інтегрується) - це дозволяє уникинути проблеми \
задавати параметри на кожному вузлі окремо. В архітектурі системи, що була обрана, компоненти віддаленої памʼяті отримують параметри роботи від вузла керування під час роботи системи. \
Початкові параметри задаються в момент ініціалізації віддаленої памʼяті та можуть змінюватись під час її роботи. Зміна параметрів під час роботи є необхідною, так як поведінка та закономірності \
роботи з памʼяттю програмного забезпечення як правило змінюються у часі і система повинна адаптувати параметри з урахуванням нової інформації."),
        paragraph("На рисунку 2.1 схематично зображено компоненти з яких складається програмне забезпечення для надання віддаленої памʼять та звʼязки між ними."),
        Block::Image(ImageBlock::new("images/components.jpg".to_owned(), "Схематичне зображення компонентів віддаленої памʼяті, звʼязків між ними та переміщення проміжків".to_owned())),
        paragraph(vec![
            "Кожен з цих компонентів надається користувачу як виконуваний файл для операційної системи Linux (за винятком клієнту віддаленої памʼяті який може інтегруватися у вузел \
обчислення за допомогою бібліотеки або окремого сервісу, що взаємодіє з механізмами керування памʼяттю операційної системи). Також надаються ".into(),
            reference("Docker", Reference::for_website("Docker", "https://www.docker.com/")),
            " контейнери та конфігурація ".into(),
            reference("Kubernetes", Reference::for_website("Kubernetes", "https://kubernetes.io/")),
            ", що спрощує розгортання у кластері. До середовища, в якому розгортається система, ставиться вимога що всі вузли доступні всім іншим вузлам за мережею.".into()
        ]),
        paragraph(vec![
            "При використанні реалізації відалленої памʼяті на практиці виникає потреба у інтеграції з зовнішними системами моніторингу для того, щоб одночасно контролювати як роботу \
віддаленої памʼяті, так і програмного забезпечення у яке вона інтегрована. Для цього, у кожному з компонентів віддаленої памʼяті передбачено відправку метрик у окремому потоці через регулярні\
проміжки часу у ".into(),
        reference("Prometheus", Reference::for_website("Prometheus - Monitoring system & time series database", "https://prometheus.io/")),
        " або інші сховища метрик сумісні з ним. Для відправки метрик використовується push-механізм, так як це робить комунікацію з зовнішньою системою моніторингу більш \
простою, прибираючи необхідність у використанні механізму Service Discovery.".into(),
        ]),
        paragraph("Так як вузли обчислення підключаються одночасно до декількох вузлів зберігання та до вузла керування, то виникає потреба в можливості ідентифікації сесії роботи з віддаленою \
памʼяттю. Це також потрібно для того, щоб у разі закриття та повторного відкриття зʼєднання сервер міг встановити що клієнт є тим самим екземпляром програмного забезпечення. Для вирішення \
цієї проблеми, у протокол передачі інформації між вузлами додається ідентифікатор сесії (run id), котрий генерується на вузлі обчислення під час запуску програми та ініціалізації клієнту \
віддаленої памʼяті і залишається незмінним увесь час роботи програми."),
        paragraph("Крім цього, вузел обчислення при встановленні зʼєднання з вузлом керування крім run id також передає таку інформацію як тип та версія програмного забезпечення. Це потрібно \
для моніторингу та збору статистики для автоматичної оптимізації параметрів віддаленої памʼяті."),
        // can tell something about the expectations (робляться наступні припущення щодо середовища):
        /* unordered_list(vec![
            "Вважається, що усі вузли системи розміщені у межах одного центру обробки даних та мають низькі мережеві затримки при спілкуванні між собою".to_owned(),
            "Мережа працює стабільно і між будь-якими двома вузлами в кластері є можливість встановити зʼєднання. Оскільки в багатьох інших задачах існує таке саме припущення (наприклад, у розподілених базах даних) і враховуючи той факт, що у межах одного центру обробки даних мережа зазвичай достатньо стабільна, то використання цього припущення не повинно накладати значних обмежень на середовища, в яких це програмне рішення може використатися".to_owned(),
            "Будь-яка розгорнута клієнтська інтеграція має можливість підключитися до сервісу керування кластером за призначеною йому IP адресою в мережі та номером порту".to_owned(),
            "Будь-яка розгорнута клієнтська інтеграція, а також сервіс керування кластером мають можливість підключитись до будь-якого розгорнутого сервісу зберігання блоків за призначеними їм IP адресами в мережі та номером порту".to_owned(),
            "Будь-який розгорнутий сервіс зберігання блоків даних має можливість відкрити зʼєднання з сервісом керування кластером за призначеною йому IP адресою в мережі та номером порту".to_owned(),
            "Для налаштування та додавання у кластер нової клієнтської інтеграції чи сервісу зберігання даних користувачу достатньо вказати IP адресу та порт сервісу керування кластером".to_owned(),
        ]),*/
        // finally, tell about async pipeline to collect events.
        // tell that manager node allows to control things a bit (for example, schedule maintenance).
        // explain how components communicate, why bincode should be used.
        // explain which methods and required to and which are optional to implement in backends.
        // explain how multiple clients work with the same storage.
        // explain how prepend works.
    ])
}
