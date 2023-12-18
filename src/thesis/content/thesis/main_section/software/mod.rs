use {
    crate::thesis::engine::{Block, subsection_header, paragraph, SubsectionHeaderBlock, TextSpan},
    super::super::applications::{DEPLOYMENT_DIAGRAM, COMPONENT_DIAGRAM, CLASS_DIAGRAM, ACCESS_SEQUENCE, BACKGROUND_THREAD_SEQUENCE},
};

mod requirements;
mod tools;

pub fn software() -> Block {
    Block::Multiple(vec![
        requirements::requirements(),
        tools::tools(),

        subsection_header("Архітектура програмного забезпечення"),
        subsection_header(SubsectionHeaderBlock::new("Компоненти програмного забезпечення що надає віддалену памʼять".to_owned()).with_level(2)),
        paragraph("Як було зазначено раніше, цей метод надання віддаленої памʼяті використовує три компоненти: інтеграція у програмне забезпечення на \
стороні вузлів обчислення, вузли зберігання та вузел керування. "),
        paragraph(TextSpan::Multiple(vec![
            "Схема структурна розгортання цих компонентів наведена у додатку ".into(),
            TextSpan::ApplicationReference(DEPLOYMENT_DIAGRAM),
            ".".into(),
        ])),
        paragraph("Інтеграція у програмне забезпечення представлена клієнтською бібліотекою або віртуальним блоковим присторєм. Інтеграція налаштовується \
розробником інформаційної системи яка використовує віддалену памʼять. У разі використання бібліотеки, цей компонент розгортається у кількості екземплярів \
рівній кількості екземплярів програмного забезпечення інформаційної системи. При використанні блокового пристроя - в залежності від кількості серверів \
на яких розміщена інформаційна система та кількості блокових пристроїв на кожному з них. Очікується що під час роботи інформаційної системи кількість \
вузлів обчислень може змінюватись."),
        paragraph("Вузел керування та вузли зберігання розгортаються на інших серверах. Вузел керування завжди один, а кількість вузлів зберігання \
може змінюватись адміністратором інформаційної системи або планувальником задач, що використовується, в залежності від обсягу вільних ресурсів. На \
одному сервері може одночасно бути розгорнуто декілька вузлів зберігання."),

        subsection_header(SubsectionHeaderBlock::new("Взаємодія компонентів".to_owned()).with_level(2)),
        paragraph(TextSpan::Multiple(vec![
            "Схема структурна компонентів програмного забезпечення що надає віддалену памʼять наведено у додатку ".into(),
            TextSpan::ApplicationReference(COMPONENT_DIAGRAM),
            ". Ця схема окрім компонентів також показує звʼязки між ними, а саме:".into(),
        ])),
        Block::UnorderedList(vec![
            "вузли зберігання передають на вузел керування інформацію про кількість вільної памʼяті".to_owned(),
            "вузел керування передає вузлам обчислення призначені їм вузли зберігання та кількість доступної памʼяті на них".to_owned(),
            "вузли зберігання та обчислення передають інформацію про свій стан через регулярні інтервали часу та вузел керування".to_owned(),
            "вузли обчислення передають проміжки памʼяті на зберігання вузлам обчислення та за запитом отримують ці дані назад".to_owned(),
        ]),
        paragraph("Передача даних між вузлами виконується по протоколу TCP."),

        subsection_header(SubsectionHeaderBlock::new("Структура клієнта віддаленої памʼяті".to_owned()).with_level(2)),
        paragraph(TextSpan::Multiple(vec![
            "Структура бібліотеки клієнта віддаленої памʼяті представлена у вигляді діаграми класів у додатку ".into(),
            TextSpan::ApplicationReference(CLASS_DIAGRAM),
            ". На діаграмі представлені:".into(),
        ])),
        Block::UnorderedList(vec![
            "клієнт віддаленої памʼяті (FarMemoryClient), з яким взаємодіє інформаційна система".to_owned(),
            "проміжки памʼяті (FarMemorySpan) та набори байтів що вони використовують (FarMemoryData)".to_owned(),
            "бекенд зберігання на вузлі зберігання (NetworkNodeBackend)".to_owned(),
            "бекенд зберігання на SSD диску (LocalDiskBackend)".to_owned(),
            "бекенд зберігання у локальній памʼяті (InMemoryBackend)".to_owned(),
            "кодування стиранням (ErasureCodingBackend)".to_owned(),
            "реплікація (ReplicationBackend)".to_owned(),
            "стиснення даних (CompressionBackend)".to_owned(),
            "шифрування даних (EncryptionBackend)".to_owned(),
            "розумні показчики FarMemory<T> і FarMemoryLocal<T> для зберігання обʼєктів".to_owned(),
            "розумні показчики FarMemorySerialized<T> і FarMemorySerializedLocal<T> для зберігання обʼєктів з використанням серіалізації".to_owned(),
            "буфер байтів (FarMemoryBuffer)".to_owned(),
            "реалізації структури даних вектор, адаптованих для роботи з віддаленою памʼяттю (FarMemoryVec<T>, FarMemoryBufferedVec<T>, FarMemorySerializedVec<T>)".to_owned(),
            "реалізація структури даних хеш-таблиця, адаптована для роботи з віддаленою памʼяттю (FarMemoryHashMap)".to_owned(),
            "реалізація алгоритмів заміщення проміжків: RandomReplacementPolicy, LeastRecentlyUsedreplacementPolicy, MostRecentlyUsedReplacementPolicy, ReplayReplacementPolicy".to_owned()
        ]),
        // can also add class diagram for far memory manager and storage later if I will need more pages.

        subsection_header(SubsectionHeaderBlock::new("Послідовність доступу до даних у віддаленій памʼяті".to_owned()).with_level(2)),
        paragraph(TextSpan::Multiple(vec![
            "Схема структурна послідовності доступу до обʼєкту що зберігається у віддаленій памʼяті інформаційною системою представлена у додатку ".into(),
            TextSpan::ApplicationReference(ACCESS_SEQUENCE),
            ". На схемі показано випадок коли обʼєкт зберігається за допомогою FarMemory<T>, знаходиться у віддаленій памʼяті та при доступі до нього \
виникає потреба у звільненні додаткової локальної памʼяті.".into(),
        ])),

        subsection_header(SubsectionHeaderBlock::new("Послідовність роботи фонового потоку клієнта віддаленої памʼяті".to_owned()).with_level(2)),
        paragraph(TextSpan::Multiple(vec![
            "Схема структурна послідовності роботи фонового потоку переміщення проміжків представлена у додатку ".into(),
            TextSpan::ApplicationReference(BACKGROUND_THREAD_SEQUENCE),
            ". На схемі показано випадок коли фоновий потік звільняє локальну памʼять через переміщення проміжків у віддалену памʼять, а після цього - \
переміщує проміжок у локальну памʼять для зменшення блокування основого потоку виконання. Проміжки обираються через використання алгоритму заміщення \
що спирається на статистику доступів до памʼяті що була зібрана під час роботи програмного забезпечення.".into(),
        ])),

        // - специфікація функцій (only add if I need some more pages, lol).

        subsection_header("Інструкція користувача"),
        paragraph("Користувачем програмного забезпечення що надає віддалену памʼять є розробник інформаціної системи у яку віддалена памʼять інтегрується. \
Рекомендованим методом інтеграції віддаленої памʼяті у програмне забезпечення є використання клієнтської бібліотеки. Саме цей спосіб розглядається далі \
в цій інструкції"),
        paragraph("Першим кроком є розгортання вузла керування. Для цього, користувач повинен ..."),
        // tell how users are expected to install and operate far memory. tell a bit about deployment as well. tell about options to use Kubernetes.

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
