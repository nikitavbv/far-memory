use crate::engine::{Block, paragraph, subsection_header};

/**
 * "AIFM: High-Performance, Application-Integrated Far Memory"
 * see: https://www.usenix.org/system/files/osdi20-ruan.pdf
 * (currently at page 8)
 * 
 * - because data structures convey their semantics to the runtime, AIFM supports custom prefetching remote data
 * in remotable list and streaming of remote data the avoid the polluting the local memory cache.
 * - application-integrated far memory avoids I/O amplification (swapping a full 4KB memory page independent of the
 * object's actual memory size).
 * - API for data structure developers to build remoteable data structures.
 * - remote servers may run a counterpart AIFM runtime and perform custom logic over data structures, avoidinig 
 * multiple roundtrips.
 * 
 * core abstractions:
 * - remoteable pointers.
 * - derefernce scopes.
 * - evacuation handles.
 * - remote devices.
 * 
 * latency:
 * - hot path for local access is carefully optimized and takes five x86-64 machine instructions.
 * - hotness tracking - when dereferencing, hot bit of a pointer is set. Under memory pressure, this is taken into
 * account when choosing which objects to evacute to remote memory.
 * - FSM is used to predict future accesses (it is updated on each dereference). When pattern is detected, prefetcher
 * threads swap in objects from the remote server. With enough prefetching, application threads always access local
 * memory.
 * - kernel bypass networking.
 * - green threads to avoid expensive context switching.
 */
pub fn aifm() -> Block {
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
    )
}