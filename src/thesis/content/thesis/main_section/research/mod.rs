use {
    crate::thesis::engine::{Block, ImageBlock, subsection_header, paragraph, unordered_list},
    self::{
        aifm::aifm,
        carbink::carbink,
        far_memory_in_warehouse_scale::far_memory_in_warehouse_scale,
        hydra::hydra,
        rdma::rdma,
    }
};

mod aifm;
mod carbink;
mod far_memory_in_warehouse_scale;
mod hydra;
mod rdma;

pub fn research() -> Block {
    Block::Multiple(vec![
        Block::SectionHeader("Огляд існуючих методів надання віддаленої памʼяті".to_owned()),
        Block::SubsectionHeader("Ресурси обладнання у розподілених системах та проблема їх ефективного використання".to_owned()),
        paragraph(r#"Будь-який сучасний центр обробки даних складається з великої кількості серверного та мережевого обладнання. На цьому обладнанні виконується програмне забезпечення, що обробляє запити від користувачів та 
може бути частинами розподілених систем."#.to_owned()),
        paragraph("Під час своєї роботи на цьому обладнанні, програмне забезпечення може використовувати наступні його ресурси:".to_owned()),
        Block::UnorderedList(vec![
            "процесорний час".to_owned(),
            "оперативна памʼять".to_owned(),
            "постійна памʼять на різних типах сховища, таких як жорсткі диски, твердотільні накопичувачі на ін.".to_owned(),
            "спеціалізовані пристрої, такі як графічні прискорювачі".to_owned(),
        ]),
        paragraph("Для кожного з цих ресурсів існує проблема їх ефективного використання та різні рішення для досягнення такої мети.".to_owned()),
        paragraph(r#"Один з методів який дозволяє підвищити ефективність використання ресурсів процесору є “надмінна підписка” (oversubscription) його обчислювального часу. Це означає що на одному процесорі запускається декілька різних 
програм або віртуальних машин, кожна з яких використовує його частину часу, а разом всі - використовують процесор майже весь час, при цьому розрахунок йде на те, що піки завантаженості цих програм не співпадають."#.to_owned()),
        paragraph(r#"Через особливості того, як програмне забезпечення працює з оперативною памʼяттю, вона є найбільш складним ресурсом, ефективність використання якого можна було б підвищити. Одним з підходів, що останнім часом багато 
досліджується та розглядається операторами великих центрів обробки даних для інтеграції є віддалена памʼять (Far Memory)."#.to_owned()),
        paragraph("Суть цього методу полягає в тому, що сервери у центрі обробки данних (і програмне забезпечення, що на них розгорнуте) можна поділити на два типи:".to_owned()),
        Block::UnorderedList(vec![
            "сервери, на яких більша частина памʼяті є вільною".to_owned(),
            "сервери, які могли б цю памʼять використовувати, якщо мали би до неї доступ".to_owned(),
        ]),
        paragraph(r#"Програмне забезпечення першого типу зазвичай має “вузьке місце” у ресурсах процесору (наприклад, виконує задачі кодування даних, або простого обміну даними), програмне забезпечення другого - у ресурсах памʼяті 
(зазвичай це аналіз великих масивів даних або просто у програмного забезпечення є деякий великий набір даних, який йому потрібен для роботи). Використання памʼяті диску для розширення основної памʼяті не є оптимальним - через великий час доступу (а в хмарній інфраструктурі в додаток до цього зазвичай диски не є локальними, а розміщені віддалено на окремій інфраструктурі). У порівнянні з часом доступу до диску час доступу до даних у памʼяті іншого серверу є значно меншим (хоча все ще більшим за той випадок, коли дані доступні локально)."#.to_owned()),
        Block::Image(ImageBlock::new("images/image1.png".to_owned(), "Схематичне зображення принципу роботи Far Memory block".to_owned())),
        paragraph("Це все робить використання такої віддаленої памʼяті привабливим для випадків, коли можна знайти сторінки памʼяті, доступ до яких відбувається порівняно не часто, перемістити їх у віддалену памʼять та звільнити місце для даних, доступ до яких відбувається частіше.".to_owned()),
        Block::SubsectionHeader("Огляд існуючих реалізацій віддаленої памʼяті".to_owned()),
        Block::Placeholder(
            Box::new(paragraph("Аналіз існуючих реалізацій віддаленої памʼяті має на меті проаналізувати існуючі реалізації, їх архітектуру, причини певних рішень. Ціллю є дізнатися які з вже досліджених підходів є ефективними та знайти відповіді на наступні дослідницькі питання:".to_owned())), 
            "replace this with a better intro. Generally, I need to point out what to focus on while analyzing existing implementations".to_owned()
        ),
        Block::UnorderedList(vec![
            "З яких компонентів складаються системи віддаленої памʼяті, що працюють в розподілених системах?".to_owned(),
            "Яким чином вони інтегруються в існуюче та нове програмне забезпечення?".to_owned(),
            "Що впливає на швидкодію системи та які є методи її покращення?".to_owned(),
            "За рахунок чого забезпечується відмовостійкість?".to_owned(),
        ]),

        rdma(),
        hydra(),
        far_memory_in_warehouse_scale(),
        carbink(),
        aifm(),

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

        Block::Placeholder(Box::new(subsection_header("Висновки до розділу")), "remove numbering".to_owned()),
        Block::Placeholder(
            Box::new(paragraph("У цьому розділі виконано аналіз проблеми та тематичних джерел за темою дослідження, що розглядається в цьому курсовому проекті. Було вивчено з яких складових частин складаються існуючі реалізації та як вони співпрацюють. З існуючих досліджень було інформацію про ефективність та недоліки підходів у архітектурі та керуванні віддаленої памʼяті. Інформація отримана в даному розділі буде використовуватись для розробки архітектури та реалізації програмного рішення, що розглядається у межах цього курсового проекту. В результаті проведеного аналізу сформульована постановка задачі, наведене призначення цілі та задачі розробки.")),
            "improve conclusions".to_owned(),
        ),
    ])
}