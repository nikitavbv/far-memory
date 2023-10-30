use crate::thesis::engine::{Block, section_header, subsection_header, paragraph, SubsectionHeaderBlock};

mod components;

pub fn far_memory_method() -> Block {
    Block::Multiple(vec![
        section_header("Методи та засоби надання віддаленої памʼяті"),
        paragraph("Загалом, принцип роботи віддаленої памʼяті полягає в тому, що програмне забезпечення передає у віддалену памʼять дані для зберігання, а коли до цих даних потрібен доступ, то \
реалізація віддаленої памʼяті за запитом від застосунку переміщує дані з інших більш повільних пристроїв зберігання даних у локальну оперативну памʼять. Після переміщення, програмне \
забезпечення працює з даними так само, як і з будь-якими іншими даними у оперативній памʼяті. Після того, як робота з даними закінчена, вони знов переміщуються на зберігання у інший клас памʼяті. \
Саме це надає можливість зменшити використання оперативної памʼяті вузла та обробляти більше даних ніж обʼєм памʼяті прозоро для програмного забезпечення (тобто без значних змін у код, та те, як \
застосунок працює з даними)."),

        components::components(),

        subsection_header("Інтеграція у програмне забезпечення"),
        paragraph("Складність інтеграції віддаленої памʼяті у програмне забезпечення полягає у тому, що зазвичай у сучасних мовах програмування доступ до даних вібувається з припущенням що всі дані \
розміщені в локальній оперативній памʼяті. Тобто структури даних, з якими працює програма, використовують показчики на памʼять які були видані алокатором памʼяті. Це робить неможливим прозору роботу \
з даними у віддаленій памʼяті для програми, так як не є можливим створити показчик на памʼять у іншому пристрої. В операційних системах звісно існує віртуальна памʼять і, наприклад, механізм memory \
mapping, але сам по собі він не підходить для того, щоб прозоро (тобто без значних змін у код) та швидко працювати з даними у памʼяті віддалених вузлів."),
        paragraph("Можливим рішенням було б внести зміни в механізми роботи операційної системи Linux з памʼяттю. Але значним недоліком цього підходу є необхідність користувачів компілювати та \
встановлювати ядро операційної системи з необіхними змінами - що робить використання віддаленої памʼяті з такою реалізацією на практиці надто складним. Навіть якщо потрібний функціонал реалізувати \
як окремий модуль ядра, це достатньо щоб зменшити область застосування на практиці. Крім цього, недоліком такого підходу є те, що на рівні операційної системи є менше інформації про контекст \
запиту до памʼяті, який є на рівні програми. Це робить більш складним обмеження використання більш повільного типу памʼяті лише для окремих структур даних. Також це робить неможливими різні оптимізації \
які базуються на додатковій інформації яка доступна на рівні програми (наприклад, до якого саме елементу масиву чи хеш-таблиці відбувається доступ)."),
        paragraph("Через це найбільш привабливим варіантом інтеграції є абстракція у коді програмного забезпечення, яка б слугувала обгорткою над даними, що зберігаються у віддаленій памʼяті і при цьому \
дозволяла б працювати з ними так само, як це відбувається з даними у локальній оперативній памʼяті. Зручною реалізацією цього є реалізація розумного показчика (smart pointer), який би надавався \
бібліотекою клієнта віддаленої памʼяті і використовується програмним забезпеченням у яке вона інтегрується. В цій роботі розглядається створення такої бібліотеки на мові програмування Rust. Розумні показчики \
у таких мовах як Rust чи C++ імітують звичайні показчики, при цьому дають додаткові можливості: реалізація такого показчика може, наприклад, виконати додатковий код у момент, коли програма виконує доступ до \
нього."),
        paragraph("Бібліотека клієнту віддаленої памʼяті, яка розробляється в цій роботі, надає розробникам прикладного програмного забезпечення розумний показчик. При створенні цього показчика через конструктор \
передається обʼєкт, розміщення памʼяті якого с цього моменту керується клієнтом віддаленої памʼяті."),
        paragraph("Використання бібліотеки віддаленої памʼяті є найбільш оптимальним у програмному забезпеченні написаному на мові програмування Rust. Створення інтеграцій для інших мов програмування знаходиться \
за межами того, що рогляадається в цій роботі. Програмне забезпечення що написано на інших мовах програмування може використовувати біндинги (bindings) для того, щоб користуватись реалізацією цієї бібліотеки з \
мов прогрумування що підтримують створення таких біндингів. Недоліком цього підходу є необхідність додаткових змін і менш зручний інтерфейс використання."),
        paragraph("Альтернативним підходом для інтеграції віддаленої памʼяті у тих випадках, коли будь-які зміни в код програмного забезпечення не є бажаними є інтеграція за допомогою механізму підкачки у \
операціній системі Linux. Як було зазначено раніше, у цього підходу є недоліки у вигляді меншої гнучкості та додаткового контексту, який можна було б використати для підвищення швидкодії віддаленої памʼяті."),
        // tell how library and service running on the end node would be working more specifically. Tell how spans are swapped in and out here. Tell about remotable pointers, buffers, optimized data structures and streaming.
        // tell about ref-counting and identification which spans are not in use. explain how memory limits work. explain how user is supposed to use the swap file and what is done to prevent recursive swap (idk if that is
        // right name for that). tell about traces as a way to monitor the system (if that is a good topic to explain).
        // explained objects serialization.
        // explain why NBD is used for running on the end node. tell that options are a library or a service running on the end node.
        // Tell how span size is being chosen and what effect it has.

        subsection_header("Забезпечення відмовостійкості"),
        // tell about replication to remote nodes and local SSDs and erasure coding. Tell how exactly data will be restored and deleted. Explain that failure domain becomes larger when far memory is used.
        // tell about healthchecks.

        subsection_header("Забезпечення швидкодії віддаленої памʼяті"),
        // tell about optimizing network requests (why TCP (also, why nodelay is used and duplex) is used and not UDP, or http or some kind of existing RPC implementation).
        // tell about reasoning behind partial swap in/swap out. tell why compression is not used. tell why copies should be avoided. tell a bit about size classes.
        // tell about background swap in and swap out threads and how synchronization should be performed.
        // explain what is the key in minimizing latency (keeping all the needed memory locally and moving it quickly) - like explained in the docs.
        // tell that only 3 out of 5 data shards are needed to minimize latency when restoring data.
        // tell about policies to evict and pre-fetch spans (and how those use stats collected, heuristics, FSM, ML models, including RNN). explain why grouping objects in spans is effective. explain why it is important to reduce fragmentation and how it can be
        // achieved. tell about compaction.
        // explain what typical performance numbers are in various environments.
        // tell about page placement algorithms
        // explain how different software accesses memory. Tell how "ideal" policy works. Tell why "least recently used" can be a bad policy in some cases.

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        // general conclusions

        // in third section explain that a demo app was implemented to measure how well everything works and the hardware of the test environment.
    ])
}
