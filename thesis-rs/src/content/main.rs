use crate::engine::{Block, ImageBlock, subsection_header, paragraph, unordered_list};

pub fn main_content() -> Block {
    /* from https://ela.kpi.ua/bitstream/123456789/49978/1/Mahisterska_dysertatsiia.pdf:
        Перший розділ містить порівняльний аналіз актуального наукового,
        інноваційного та практичного світового та вітчизняного здобутку у чіткій
        відповідності до теми магістерської дисертації. Для з'ясування стану розробки
        обраної теми складається огляд літератури, з якого можна зробити висновок, що
        дана тема ще не розкрита (розкрита лише частково, або не в тому аспекті) і тому
        вимагає подальшого розроблення. Якщо такий висновок не випливає логічно з
        огляду, то дисертанту немає сенсу розробляти обрану тему.
        В огляді необхідно:
        − окреслити основні етапи розвитку наукової думки за обраною
        сформульованою задачею;
        − стисло, критично висвітлити роботи попередників (переваги та недоліки,
        порівняльний аналіз отриманих раніше результатів, огляд аналогів на ринку
        схожих рішень або продуктів).
        Огляд літератури за темою демонструє ґрунтовне ознайомлення
        дисертанта зі спеціальною літературою, його вміння систематизувати джерела,
        критично їх розглядати, виділяти суттєве, оцінювати зроблене раніше іншими 
        дослідниками, визначати головне у сучасному стані вивчення теми. Матеріали
        такого огляду треба систематизувати в певному логічному зв'язку і
        послідовності. Тому перелік праць (не менше 20-ти) та їх критичний розгляд не
        обов'язково подавати у хронологічному порядку. При цьому слід пам'ятати, що
        оскільки дисертація розкриває відносно вузьку тему, то огляд праць
        попередників роблять тільки з питань обраної теми, а не за проблемою в цілому.
        В огляді називають і критично оцінюють публікації, прямо і безпосередньо
        причетні до теми дисертації. Зайвим є виклад всього, що стало відомим
        дисертанту з прочитаного, і того, що побічно стосується його праці.
        Результатом проведеного аналізу має бути визначення тих питань, що
        залишились невирішеними, а отже і конкретизація даного дослідження у
        розв’язанні вказаної проблеми (завдання), та формулювання існуючих недоліків
        і напрямів їх подальшого усунення та вирішення, що забезпечує актуальність
        магістерського дослідження. Загальний обсяг огляду літератури не повинен
        перевищувати 20 % обсягу основної частини магістерської дисертації. */

    Block::Multiple(vec![
        Block::SectionHeader("Огляд існуючих методів надання віддаленої памʼяті".to_owned()),
        Block::SubsectionHeader("Ресурси обладнання у розподілених системах та проблема їх ефективного використання".to_owned()),
        Block::Paragraph(r#"Будь-який сучасний центр обробки даних складається з великої кількості серверного та мережевого обладнання. На цьому обладнанні виконується програмне забезпечення, що обробляє запити від користувачів та 
може бути частинами розподілених систем."#.to_owned()),
        Block::Paragraph("Під час своєї роботи на цьому обладнанні, програмне забезпечення може використовувати наступні його ресурси:".to_owned()),
        Block::UnorderedList(vec![
            "процесорний час".to_owned(),
            "оперативна памʼять".to_owned(),
            "постійна памʼять на різних типах сховища, таких як жорсткі диски, твердотільні накопичувачі на ін.".to_owned(),
            "спеціалізовані пристрої, такі як графічні прискорювачі".to_owned(),
        ]),
        Block::Paragraph("Для кожного з цих ресурсів існує проблема їх ефективного використання та різні рішення для досягнення такої мети.".to_owned()),
        Block::Paragraph(r#"Один з методів який дозволяє підвищити ефективність використання ресурсів процесору є “надмінна підписка” (oversubscription) його обчислювального часу. Це означає що на одному процесорі запускається декілька різних 
програм або віртуальних машин, кожна з яких використовує його частину часу, а разом всі - використовують процесор майже весь час, при цьому розрахунок йде на те, що піки завантаженості цих програм не співпадають."#.to_owned()),
        Block::Paragraph(r#"Через особливості того, як програмне забезпечення працює з оперативною памʼяттю, вона є найбільш складним ресурсом, ефективність використання якого можна було б підвищити. Одним з підходів, що останнім часом багато 
досліджується та розглядається операторами великих центрів обробки даних для інтеграції є віддалена памʼять (Far Memory)."#.to_owned()),
        Block::Paragraph("Суть цього методу полягає в тому, що сервери у центрі обробки данних (і програмне забезпечення, що на них розгорнуте) можна поділити на два типи:".to_owned()),
        Block::UnorderedList(vec![
            "сервери, на яких більша частина памʼяті є вільною".to_owned(),
            "сервери, які могли б цю памʼять використовувати, якщо мали би до неї доступ".to_owned(),
        ]),
        Block::Paragraph(r#"Програмне забезпечення першого типу зазвичай має “вузьке місце” у ресурсах процесору (наприклад, виконує задачі кодування даних, або простого обміну даними), програмне забезпечення другого - у ресурсах памʼяті 
(зазвичай це аналіз великих масивів даних або просто у програмного забезпечення є деякий великий набір даних, який йому потрібен для роботи). Використання памʼяті диску для розширення основної памʼяті не є оптимальним - через великий час доступу (а в хмарній інфраструктурі в додаток до цього зазвичай диски не є локальними, а розміщені віддалено на окремій інфраструктурі). У порівнянні з часом доступу до диску час доступу до даних у памʼяті іншого серверу є значно меншим (хоча все ще більшим за той випадок, коли дані доступні локально)."#.to_owned()),
        Block::Image(ImageBlock::new("images/image1.png".to_owned(), "Схематичне зображення принципу роботи Far Memory block".to_owned())),
        Block::Paragraph("Це все робить використання такої віддаленої памʼяті привабливим для випадків, коли можна знайти сторінки памʼяті, доступ до яких відбувається порівняно не часто, перемістити їх у віддалену памʼять та звільнити місце для даних, доступ до яких відбувається частіше.".to_owned()),
        Block::SubsectionHeader("Огляд існуючих реалізацій віддаленої памʼяті".to_owned()),
        Block::Placeholder(
            Box::new(Block::Paragraph("Аналіз існуючих реалізацій віддаленої памʼяті має на меті проаналізувати існуючі реалізації, їх архітектуру, причини певних рішень. Ціллю є дізнатися які з вже досліджених підходів є ефективними та знайти відповіді на наступні дослідницькі питання:".to_owned())), 
            "replace this with a better intro. Generally, I need to point out what to focus on while analyzing existing implementations".to_owned()
        ),
        Block::UnorderedList(vec![
            "З яких компонентів складаються системи віддаленої памʼяті, що працюють в розподілених системах?".to_owned(),
            "Яким чином вони інтегруються в існуюче та нове програмне забезпечення?".to_owned(),
            "Що впливає на швидкодію системи та які є методи її покращення?".to_owned(),
            "За рахунок чого забезпечується відмовостійкість?".to_owned(),
        ]),
        Block::SubsectionHeader("Remote Direct Memory Access та її реалізації".to_owned()),
        Block::Placeholder(
            Box::new(Block::Paragraph("Технологія віддаленого прямого доступу до памʼяті (Remote Direct Memory Access) полягає в використанні спеціальних апаратних засобів, що дозволяють вузлам в системі отримувати дані з інших вузлів з невеликою затримкою з інших вузлів без витрачання ресурсів процесору цих вузлів для обробки запитів. Однією з найбільш розповсюджених реалізацій RDMA є InfiniBand. Цей підхід використовується і є виправданим для використання в середовищах високопродуктивних обчислень (High Performance Computing - HPC). Головним недоліком цієї реалізації віддаленої памʼяті є те, що вона потребує додаткового спеціалізованого обладнання. Для задач та середовища що розглядаються в цьому курсовому проекті не є підхожим рішенням, тому що використання використання додаткових пристроїв потребує додаткових ресурсів і не вирішує проблему більш ефективного використання наявних ресурсів без змін в апаратну платформу.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::SubsectionHeader("Software-Defined Far Memory in Warehouse-Scale Computers".to_owned()),
        Block::Placeholder(
            Box::new(Block::Paragraph("Компанія Google розробила та в тестовому режимі інтегрувала систему віддаленої памʼяті в своїх центрах обробки даних. Наскільки відомо на момент проведення дослідження існуючих реалізацій для цього курсового проекту, це єдиний великий оператор центрів обробки даних, який спроектував та застосував систему віддаленої памʼяті на великих масштабах та обладнанні яке виконує програмне забезпечення, що обробляє запити від користувачів (іншими словами, на справжньому навантаженні, а не у тестовому середовищі).".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("В науково-дослідній роботі, опублікованій Google, зазначається що система для своєї роботи використовує функціонал ядра операційної системи Linux під назвою zswap. Перевагою такого рішення є те, що zswap це перевірена часом технологія, яка надає можливість компресії сторінок памʼяті. Інтеграція з цим функціоналом дозволяє переносити частини памʼяті на віддалені вузли без використання додаткового апаратного забезпечення.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Система відслідковує час останнього доступу до сторінок памʼяті для визначення “холодних” сторінок, які було б ефективно перенести у віддалену памʼять. Також окремий компонент системи відслідковує сторінки, які потрібно перенести з віддаленої памʼяті у локальну. Обидва з цих компонентів контролюються встановленим на кожен сервер агентом віддаленої памʼяті. Ця програма збирає статистику використання памʼяті та задає параметри роботи для системи.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("В публікації зазначено, що режими доступу до памʼяті постійно змінюються. Наприклад, зібрана статистика показує що кількість холодних сторінок памʼяті варіюʼться від 1% до 61% в залежності від часу дня, програмного забезпечення що розгорнуто на серверному обладнанні, запитів від користувачів та конкретного серверу. Це створює необхідність мати окремий компонент автоматичного налаштування системи. Цей компонент отримує статистику зібрану агентами на обчислювальних вузлах та за допомогою моделей машинного навчання обирає нові значення параметрів для компонентів що керують переміщенням сторінок памяʼті.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Недоліком системи є те, що її програмний код не є публічно доступним, а окремі компоненти є специфічними для середовища, що використовується в компанії Google.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::SubsectionHeader("Carbink: Fault-Tolerant Far Memory".to_owned()),
        Block::Placeholder(
            Box::new(Block::Paragraph("Carbink це також система віддаленої памʼяті розроблена та протестована компанією Google в своїх центрах обробки даних. Ця реалізація фокусується на покращені відмовостійкості та зниженні рівня затримок при роботі з віддаленою памʼяттю.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Важливим компонентом цієї реалізації є менеджер памʼяті (memory manager). Цей компонент керує розподілом фрагментів памʼяті по вузлах, що їх зберігають та перевіряє стан роботи цих вузлів. Важливим припущенням є те, що вважається що менеджер памʼяті завжди залишається в робочому стані. Мережевий звʼязок теж вважається стабільним.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Відмовостійкість забезпечується тим, що коли менеджер памʼяті виявляє недоступність одного з вузлів памʼяті (через апаратний чи програмний збій), то запускається процес відновлення. Завдяки використанню методу кодування з видаленням (erasure coding) при втраті блоку даних з одного вузла, його можна відновити за допомогою певних математичних перетворень інших даних. Перевагою такого методу є невеликий рівень надлишковості у порівнянні з рішенням, яке використовує реплікацію (тобто зберігання декілька копій даних).".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Низька затримка забезпечується головним чином за допомогою обʼєднання невеликих шматків даних у більш великі (розміром декілька мегабайт) блоки (які називаються spans). Робота з більш великими блоками знижує навантаження на мережу та час для отримання даних. Недоліком такого підходу зазначається зберігання зайвих даних у блоках, що приводить до дещо більшого (на 35%) використання памʼяті на віддалених вузлах.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Для інтеграції з клієнтським програмним забезпеченням (в якого є потреба у додатковій памʼяті) використовується бібліотека написана на мові C++ що дає доступ до памʼяті за допомогою розумних покажчиків. Це вимагає деякої зміни в програмне забезпечення.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Аналогічно до попередньої системи, програмний код для цієї реалізації не є публічно доступним. Це не дозволяє використати систему в центрах обробки даних інших операторів.".to_owned())),
            "improve this text".to_owned(),
        ),

        Block::Placeholder(
            Box::new(Block::Multiple(vec![
                subsection_header("AIFM: High-Performance Application-Integrated Far Memory"),
                paragraph("Система Application-Integrated Far Memory була розроблена дослідниками з VMWare Research. У порівнянні з реалізаціями віддаленої памʼяті компанії Google, ця система має схожі та відмінні елементи."),
                paragraph("Головною відмінною рисою цієї реалізації є інтеграція з клієнтським програмним забезпеченням за допомогою клієнтської бібліотеки на C++ що надає розумні покажчики та структури даних оптимізовані для використання з віддаленою памʼяттю."),
                paragraph("Для спілкування між сервером що надає памʼять та сервером що її потребує використовується TCP/IP зʼєднання."),
                paragraph("Для виявлення ділянок памʼяті, що можна перенести у віддалену памʼять, використовується механізм відслідковування рівня гарячесті сторінок памʼяті. Також, для підвищення ефективності, структури даних визначені бібліотекою AIFM реалізують предзванатжання наступних ділянок памʼяті."),
                paragraph("Для цієї реалізації є доступним програмний код. Недоліком є те, що без додаткової доробки він підтримує лише один сервер що надає віддалену памʼять. Система не містить компоненту для розподілення та керування памʼяттю по кластеру вузлів. Також реалізація не є придатною для розгортання в середовищах що обробляють справжнє навантаження без додаткової доробки. Система має складний процес розгортання, залежить від зовнішніх компонентів, не має простих механізмів конфігурування та моніторингу."),
            ])),
            "improve AIFM description".to_owned(),
        ),

        Block::Placeholder(
            Box::new(Block::Multiple(vec![
                subsection_header("Постановка задачі"),
                paragraph("Метою роботи є покращення інфраструктурних компонентів та інструментів, що можуть використовуватись операторами центрів обробки даних та розробниками програмного забезпечення для розгортання та використання віддаленої памʼяті. Для досягнення мети необхідно вирішити наступні задачі:"),

                unordered_list(vec![
                    "аналіз технічних рішень, що використовуються в існуючих реалізаціях, особливостей програмних та апаратних платформ, що використовуються для розгортання сучасного програмного забезпечення".to_owned(),
                    "розробка методу та архітектури програмного забезпечення для надання програмно-визначеної віддаленої памʼяті у розподілених системах".to_owned(),
                    "реалізація програмного забезпечення для надання віддаленої памʼяті а також необхідні компоненти для інтеграції його у клієнтське програмне забезпечення згідно з розробленою архітектурою".to_owned(),
                    "оцінка ефективності запропонованого рішення".to_owned(),
                ]),
                paragraph("Створене програмне забезпечення повинно відповідати наступним вимогам:"),
                unordered_list(vec![
                    "Реалізація віддаленої памяʼті повинна містити сервіс, який користувачі системи можуть розгорнути на вузлах системи (під управлінням операційної системи Linux), що мають вільну памʼять для її використання по мережі. Цей компонент повинен використовувати невелику кількість ресурсів, та для зберігання даних використовувати кількість памʼяті задану користувачем або визначену автоматично".to_owned(),
                    "Реалізація повинна мати варіанти інтеграції як в нове програмне забезпечення (де є можливість змінювати програмний код) так і в існуюче (де змінювати код не є можливим)".to_owned(),
                    "Для вирішення проблеми, що розглядається, затримка системи в операціях читання та запису повинна бути нижчою за затримки при зберіганні даних у постійному сховищі, таких як жорсткі диски та твердотільні накопичувачі, що є доступними у середовищах де ця система буде розгортатися".to_owned(),
                    "Повинен бути наявний центральний компонент, який налаштовує конфігурацію та дозволяє керувати усією системою".to_owned(),
                    "Реалізація повинна мати автоматичну зміну параметрів з урахуванням особливостей програмного забезпечення, що використовується".to_owned(),
                    "Повинна забезпечуватись відмовостійкість та збереження даних що зберігаються у разі апаратних чи програмних збоїв у кластері".to_owned(),
                    "Програмне забезпечення повинне бути простим у розгортанні,  адмініструванні а також у інтеграції в клієнтське програмне забезпечення".to_owned(),
                ]),
                paragraph("Призначенням цієї розробки є надання компонентів для розгортання кластеру віддаленої памʼяті та інструментів для її використання в існуючому та новому програмному забезпеченні."),
            ])),
            "improve task definition".to_owned(),
        ),

        Block::Placeholder(
            Box::new(Block::Multiple(vec![
                subsection_header("Висновки"),
                paragraph("У цьому розділі виконано аналіз проблеми та тематичних джерел за темою дослідження, що розглядається в цьому курсовому проекті. Було вивчено з яких складових частин складаються існуючі реалізації та як вони співпрацюють. З існуючих досліджень було інформацію про ефективність та недоліки підходів у архітектурі та керуванні віддаленої памʼяті. Інформація отримана в даному розділі буде використовуватись для розробки архітектури та реалізації програмного рішення, що розглядається у межах цього курсового проекту. В результаті проведеного аналізу сформульована постановка задачі, наведене призначення цілі та задачі розробки."),
                subsection_header("Розробка архітектури програмно-визначеної віддаленої памʼяті"),
                subsection_header("Компоненти системи"),
                paragraph("З урахуванням специфіки доменної області, вимог, що висуваються до програмного забезпечення та результатів попереднього аналізу проблеми було розроблено архітектуру, що складається з наступних компонентів:"),
                unordered_list(vec![
                    "Сервіс зберігання блоків памʼяті".to_owned(),
                    "Сервіс керування кластером віддаленої памʼяті".to_owned(),
                    "Клієнтська інтеграція".to_owned(),
                ]),
                paragraph("Центральна сутність, з якою працюють усі компоненти це блоки памʼяті."),
                paragraph("Усі компоненти програмного рішення віддаленої памʼяті розгортатимуться за допомогою бінарних виконуваних файлів призначених для операційної системи Linux, а також (за вибором користувача) з використанням Docker контейнерів, що можна розгорнути за допомогою сучасних систем оркестрації, як наприклад Kubernetes."),
                paragraph("Архітектура програмного рішення віддаленої памʼяті має наступні припущення щодо середовища, в якому вона буде розгортатися:"),
                unordered_list(vec![
                    "Вважається, що усі вузли системи розміщені у межах одного центру обробки даних та мають низькі мережеві затримки при спілкуванні між собою".to_owned(),
                    "Мережа працює стабільно і між будь-якими двома вузлами в кластері є можливість встановити зʼєднання. Оскільки в багатьох інших задачах існує таке саме припущення (наприклад, у розподілених базах даних) і враховуючи той факт, що у межах одного центру обробки даних мережа зазвичай достатньо стабільна, то використання цього припущення не повинно накладати значних обмежень на середовища, в яких це програмне рішення може використатися".to_owned(),
                    "Будь-яка розгорнута клієнтська інтеграція має можливість підключитися до сервісу керування кластером за призначеною йому IP адресою в мережі та номером порту".to_owned(),
                    "Будь-яка розгорнута клієнтська інтеграція, а також сервіс керування кластером мають можливість підключитись до будь-якого розгорнутого сервісу зберігання блоків за призначеними їм IP адресами в мережі та номером порту".to_owned(),
                    "Будь-який розгорнутий сервіс зберігання блоків даних має можливість відкрити зʼєднання з сервісом керування кластером за призначеною йому IP адресою в мережі та номером порту".to_owned(),
                    "Конфігурація усієї системи знаходиться та редагується користувачем в сервісі керування кластером віддаленої памʼяті. Для налаштування та додавання у кластер нової клієнтської інтеграції чи сервісу зберігання даних користувачу достатньо вказати IP адресу та порт сервісу керування кластером".to_owned(),
                    "Для реалізації усіх компонентів системи використовується мова програмування Rust".to_owned(),
                ]),
                subsection_header("Блоки памʼяті"),
                paragraph("Блоками памʼяті в цьому програмному рішенні є набором байт, які обробляються системою як єдина одиниця. Розмір блоку складає 4 мегабайти за замовчуванням або розмір вказаний користувачем при розгортанні системи. Як і в деяких існуючих реалізаціях, ціль використання блоків полягає в обʼєданні невеликих фрагментів памʼяті у більш великі блоки для більш швидкої та ефективної обробки, зниження затримки в операцій читання та запису."),
                paragraph("Кожному блоку памʼяті призначається унікальний ідентифікатор (ID), який є цілим числом, яке займає 64 біти памʼяті."),
                subsection_header("Cервіс зберігання блоків"),
                paragraph("Сервіс зберігання блоків - сервіс, що розгортається на вузлах розподіленої системи, що містять вільну памʼять для надання доступу до неї по мережі."),
                paragraph("Інтерфейсом цього компоненту є сховище ключ-значення, де ключем є ідентифікатор блоку памʼяті, а значенням - байти блоку."),
                subsection_header("Сервіс керування кластером"),
                paragraph("Сервіс керування кластером приймає від розгорнутих сервісів зберігання блоків інформацію про кількість блоків, яку вони можуть отримати для зберігання. Далі, за запитами від клієнтських інтеграцій, сервіс керування кластером призначає їм ідентифікатори блоків для використання в клієнтському програмному забезпеченні, а також адреси сервісів зберігання блоків, на яких вони розміщені."),
                paragraph("Для забезпечення відмовостійкості, користувач обирає один зі способів забезпечення відновлення даних у разі програмних чи апаратних збоїв:"),
                unordered_list(vec![
                    "Реплікація (за вказаним користувачем фактором n) - цей алгоритм створює n ідентичних копій даних на різних вузлах системи. У разі якщо один з вузлів вийде з кластеру, то дані будуть відновлені з інших вузлів, де зберігаються репліковані копії.".to_owned(),
                    "Кодування стиранням (erasure coding) - цей алгоритм ділить блоки даних на менші блоки, кожен з яких має надмірність (конфігурація алгоритму задається користувачем в залежності від його вимог до відмовостійкості кластеру). У разі відмови одного з вузлів кластеру, втрачену інформацію можна відновити з інших частин виконавши нескладні математичні перетворення.".to_owned(),
                ]),
                paragraph("Сервіс керування кластером також надає інструменти для моніторингу, перегляду стану компонентів та внесення змін в кластер. Надається веб-інтерфейс користувача для перегляду інформації. Для інтеграції з зовнішньою системою моніторинга надається HTTP інтерфейс для передачі метрик та статистики у форматі Prometheus."),
                paragraph("Крім цього, на сервісі керування кластером полягає роль автоматичної зміни конфігурації кластеру на основі статистики зібраної клієнтськими інтеграціями. Ці налаштування мають вид правил, які відправляються на клієнтські інтеграції та задають їм за яких умов необхідно виконати предзавантаження окремих блоків даних для зниження часу затримки."),
                subsection_header("Клієнтська інтеграція"),
                paragraph("Як зазначалось раніше, клієнтська інтеграція має два варіанти для розгортання на вибір користувача (в кластері можно використовувати обидва одночасно)."),
                subsection_header("Бібліотека на мові програмування Rust для інтеграції в клієнтське програмне забезпечення"),
                paragraph("Цей спосіб використання підходить для програмного забезпечення, код якого є можливість змінити та яке написане на мові програмування Rust. Ця інтеграція є рекомендованою для використання, оскільки більш ефективна в роботі та має більш низькі затримки."),
                paragraph("Для її використання, користувач додає в своє програмне забезпечення бібліотеку, яка надає йому наступні інструменти для використання в своєму коді:"),
                unordered_list(vec![
                    "Розумний показчик FarMemory<T>, який за допомогою механізмів Deref<T> та Drop<T>, наданих мовою програмування Rust, завантажує дані та відправляє до віддаленою памʼяті за необхідністю.".to_owned(),
                    "Структура даних FarMemoryList<T> яка реалізує інтерфейс списку та зберігає дані у віддаленій памʼяті".to_owned(),
                    "Структура даних FarMemoryTable<K, V> яке реалізує інтерфейс хеш-таблиці та зберігає дані у віддаленій памʼяті".to_owned(),
                    "Функцію allocate_far_memory_block, яка резервує блок у віддаленій памʼяті та повертає його ідентифікатор. Ця функція призначена для випадків коли розробнику потрібен низькорівневий доступ до памʼяті".to_owned(),
                    "Функції read_far_memory_block та write_far_memory_block що дозволяють прочитати та записати зміст блоку у віддаленій памʼяті за його ідентифікатором. Ці функція призначена для випадків коли розробнику потрібен низькорівневий доступ до памʼяті".to_owned(),
                    "Функція free_far_memory_block, що визволяє з використання блок памʼяті за його ідентифікатором. Ця функція призначена для випадків коли розробнику потрібен низькорівневий доступ до памʼяті".to_owned(),
                ]),
                subsection_header("Сервіс клієнтської інтеграції"),
                paragraph("Цей варіант використання призначено для випадків, коли немає можливості змінити програмний код клієнтського програмного забезпечення, або коли воно використовує інші мови програмування, що не дозволяє використати бібліотеку клієнтської інтеграції."),
                paragraph("Цей спосіб полягає в розгортанні сервісу на тому ж обладнанні, яке виконує програмне клієнтське програмне забезпечення. Сервіс використовує відповідний функціонал операційної системи Linux для створення віртуального блокового пристрою. Далі цей блоковий пристрій використовується клієнтським забезпеченням для прямого зберігання даних (за необхідністю, на блоковому пристрою розміщується файлова система) або на блоковому пристрою розміщується файл підкачки, в який операційна система при низькому рівні вільної памʼяті, автоматично переносить сторінки памʼяті."),
                subsection_header("Взаємодія компонентів"),
                paragraph("Для комунікації між компонентами використовується TCP/IP зʼєднання. Для кодування повідомлень має сенс використати Protocol Buffers. Це дозволить пересилати дані між компонентами з низькими затримками та додатковими витратами."),
                Block::Image(ImageBlock::new("images/image2.jpg".to_owned(), "Схема компонентів кластеру та потоки даних між ними".to_owned())),
                subsection_header("Висновки"),
                paragraph("В цьому розділі було формалізовано вимоги до програмного рішення, що розглядається в межах цього курсового проекту. Грунтуючись на цих вимогах, було розроблено архітектуру програмно визначеної віддаленої памʼяті призначеної для розгортання у сучасних розподілених системах. Ця архітектура та описані деталі її реалізації будуть в подальшому використовуватись при реалізації, тестуванні та впровадженні цього програмного рішення."),
            ])),
            "process this text".to_owned(),
        ),
    ])
}