use crate::thesis::engine::{Block, section_header, paragraph, SubsectionHeaderBlock};

pub fn evaluation() -> Block {
    Block::Multiple(vec![
        section_header("Статистичне дослідження ефективності запропонованого методу"),
        paragraph("Статистичне дослідження ефективності запропонованого методу має на меті отримати відповіді на наступні запитання:"),
        Block::UnorderedList(vec![
            "Яким є рівень швидкодії типового програмного забезпечення з різними схемами доступу до оперативної памʼяті при викиорстанні віддаленої \
памʼяті?".into(),
            "Як залежить швидкодія програмного забезпечення що використовує віддалену памʼять від розподілу запитів до обʼєктів у памʼяті?".into(),
            "Яким є рівень швидкодії при використанні різних алгоритмів заміщення проміжків?".into(),
        ]),

        paragraph("Для того, щоб оцінити рівень швидкодії для різних типів програмного забезпечення, конфігурацій клієнта віддаленої памʼяті та \
алгоритмів заміщення проміжків, була проведена серія статистичних досліджень. Для цих досліджень було використано два сервери з наступним \
апаратним забезпеченням: процесор AMD Ryzen 5 3600, 64 Гб оперативної памʼяті, мережева карта Intel 82599 пропускною здатністю 10 Гбіт/c. \
При цьому, між серверами встановлено пряме мережеве зʼєднання: мережеві карти зʼєднані напряму, без використання додаткового мережевого обладнання, \
таких як маршрутизатори чи комутатори. На цих серверах встановлена операційна система ArchLinux з версією ядра 6.5."),
        paragraph("Віддалена памʼять бьула інтегрована у три синтетичні інформаційні системи, що були розроблені для цього дослідження."),
        paragraph("Першою такою інформаційною системою є програмне забезпечення для генерації тексту за допомогою великої мовленнєвої моделі Llama2. \
За основу була взята існуюча реалізація з відкритим кодом цієї програми на мові програмування Rust (llama2-rs). У віддалену \
памʼять було переміщено ваги нейронної мережі. Ця програма в циклі обробляє запити на \
генерацію наступного токена тексту. Показником швидкодії для цього програмного забезпечення є кількість згенерованих токенів у хвилину. Особливістю \
цієї програми є те, що під час генерації тексту відбувається циклічний доступ до усіх параметрів нейронної мережі у незмінному порядку. Це означає що \
цю програму можно віднести до класу програмного забезпечення, що багато разів сканує весь робочий набір даних у незмінному порядку."),
        paragraph("Крім цього, було реалізовано інформаційну систему веб сервісу, що обробляє запити від користувача. Ця програма є подібною до тієї, що \
використовувалась для оцінки ефективності у AIFM. Ця програма на початку своєї роботи генерує великий масив зображень, кожне з яких має розмір 8Кб. Крім \
цього, генерується набір користувачів, кожному з яких ставиться у відповідність одне з зображень. Обидві структури даних (масив та хеш-таблиця) \
зберігаються у віддаленій памʼяті. Під час своєї роботи, ця інформаційна система обробляє запити, кожен з яких складається з 32 випадкових ідентифікаторів \
користувачів. За кожним ідентифікатором, ..."), // todo: finsih describing second app
        // todo: describe third app
        // todo: explain that performance is throughput
        // todo: explain that 0.2 is because of network/CPU throughput.
        // todo: tell that I also was able to verify that integration is simple enough with llm inference application.

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
