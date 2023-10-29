use crate::thesis::engine::{Block, section_header, subsection_header, paragraph, unordered_list, SubsectionHeaderBlock, ImageBlock};

pub fn far_memory_method() -> Block {
    Block::Multiple(vec![
        section_header("Методи та засоби надання віддаленої памʼяті"),
        // write some kind of an intro here

        subsection_header("Компоненти системи"),
        // tell about manager node (also, explain that it is only used when there are a lot of nodes, not one and not SSD/in-memory), storage nodes (that it is basically kv storage. Also explain why Redis does not make
        // much sense here) and integration (using library or service running on the end node - explain why NBD is used). Tell what span is here (and how its size is being chosen and what effect it has).
        // Tell about span ids. Probably tell a bit how system can be deployed. Also, tell about metrics collection. Tell about run IDs and how those are used. can tell something about the expectations:
        /* unordered_list(vec![
            "Вважається, що усі вузли системи розміщені у межах одного центру обробки даних та мають низькі мережеві затримки при спілкуванні між собою".to_owned(),
            "Мережа працює стабільно і між будь-якими двома вузлами в кластері є можливість встановити зʼєднання. Оскільки в багатьох інших задачах існує таке саме припущення (наприклад, у розподілених базах даних) і враховуючи той факт, що у межах одного центру обробки даних мережа зазвичай достатньо стабільна, то використання цього припущення не повинно накладати значних обмежень на середовища, в яких це програмне рішення може використатися".to_owned(),
            "Будь-яка розгорнута клієнтська інтеграція має можливість підключитися до сервісу керування кластером за призначеною йому IP адресою в мережі та номером порту".to_owned(),
            "Будь-яка розгорнута клієнтська інтеграція, а також сервіс керування кластером мають можливість підключитись до будь-якого розгорнутого сервісу зберігання блоків за призначеними їм IP адресами в мережі та номером порту".to_owned(),
            "Будь-який розгорнутий сервіс зберігання блоків даних має можливість відкрити зʼєднання з сервісом керування кластером за призначеною йому IP адресою в мережі та номером порту".to_owned(),
            "Конфігурація усієї системи знаходиться та редагується користувачем в сервісі керування кластером віддаленої памʼяті. Для налаштування та додавання у кластер нової клієнтської інтеграції чи сервісу зберігання даних користувачу достатньо вказати IP адресу та порт сервісу керування кластером".to_owned(),
            "Для реалізації усіх компонентів системи використовується мова програмування Rust".to_owned(),
        ]),*/
        // finally, tell about async pipeline to collect events.
        // tell that manager node allows to control things a bit (for example, schedule maintenance).
        // explain how components communicate, why bincode should be used.
        // explain which methods and required to and which are optional to implement in backends.
        // explain how multiple clients work with the same storage.
        // explain how prepend works.

        subsection_header("Інтеграція у програмне забезпечення"),
        // tell how library and service running on the end node would be working more specifically. Tell how spans are swapped in and out here. Tell about remotable pointers, buffers, optimized data structures and streaming.
        // tell about ref-counting and identification which spans are not in use. explain how memory limits work. explain how user is supposed to use the swap file and what is done to prevent recursive swap (idk if that is
        // right name for that). tell about traces as a way to monitor the system (if that is a good topic to explain).
        // explained objects serialization.

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
