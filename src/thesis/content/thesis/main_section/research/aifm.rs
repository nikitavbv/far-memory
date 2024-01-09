use crate::thesis::engine::{Block, paragraph, subsection_header, Reference, reference};

/**
 * "AIFM: High-Performance, Application-Integrated Far Memory"
 * see: https://www.usenix.org/system/files/osdi20-ruan.pdf
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
 * - pauseless evacuation.
 *
 * integration:
 * - evacuator maintains local free memory ratio above the specified value (0.12 by default).
 * - activities necessary for evacuation are prioritized by scheduler to avoid running out of memory.
 * - remoteable data structures: array, vector, list, stack, queue, hashtable (table index is local, but key-value
 * is remote).
 * - TCP-remote backend and SSD backend.
 */
pub fn aifm() -> Block {
    Block::Multiple(vec![
        subsection_header("AIFM: High-Performance Application-Integrated Far Memory"),
        paragraph(vec![
            "Система ".into(),
            reference("Application-Integrated Far Memory (AIFM)", Reference::for_publication(
                "AIFM: High-Performance, Application-Integrated Far Memory".to_owned(),
                "Zhenyuan Ruan, Malte Schwarzkopf, Marcos K. Aguilera, Adam Belay".to_owned(),
                2020,
                "14th USENIX Symposium on Operating Systems Design and Implementation (OSDI 20)".to_owned(),
            )),
            " на відміну від попередніх реалізацій віддаленої памʼяті не потребує спеціалізованого апаратного забезпечення, для передачі даних використовується TCP.".into(),
        ]),

        // TODO: improve
        paragraph("Головною відмінною рисою цієї реалізації є інтеграція з клієнтським програмним забезпеченням за допомогою клієнтської бібліотеки на C++ що надає розумні покажчики та структури даних оптимізовані для використання з віддаленою памʼяттю."),
        paragraph("Для спілкування між сервером що надає памʼять та сервером що її потребує використовується TCP/IP зʼєднання."),
        paragraph("Для виявлення ділянок памʼяті, що можна перенести у віддалену памʼять, використовується механізм відслідковування рівня гарячості сторінок памʼяті. Також, для підвищення ефективності, структури даних визначені бібліотекою AIFM реалізують предзавантаження наступних ділянок памʼяті."),
        paragraph("Для цієї реалізації є доступним програмний код. Недоліком є те, що без додаткової доробки він підтримує лише один сервер що надає віддалену памʼять. Система не містить компоненту для розподілення та керування памʼяттю по кластеру вузлів. Також реалізація не є придатною для розгортання в середовищах що обробляють справжнє навантаження без додаткової доробки. Система має складний процес розгортання, залежить від зовнішніх компонентів, не має простих механізмів конфігурування та моніторингу."),
    ])
}
