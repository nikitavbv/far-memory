use crate::thesis::engine::{Block, paragraph, subsection_header, reference, Reference};

/**
 * "Carbink: Fault-tolerant Far Memory"
 * see: https://www.usenix.org/system/files/osdi22-zhou-yang.pdf
 *
 * - erasure-coding
 * - remote memory compaction
 * - swapping at granularity of spans because erasure coding requires equal-sized blocks.
 * - grouping cold objects together and hot objects together.
 * - one-sided remote memory accesses to minimize network utilization.
 * - erasure coding scheme allows to fetch a far memory region using a single network request.
 * - runs pauseless defragmentation threads in the background to solve the fragmentation in far RAM.
 * - allows computation to be offloaded to remote memory nodes.
 * - remote compaction - threads running on compute nodes find pairs of spans to create a new spanset without dead
 * space.
 *
 * terms:
 * - span - contiguous set of pages that contain objects of the same size class (configuration parameters borrowed
 * from TCMalloc).
 * - region - 1GB or larger.
 * - region table - maps the Region ID of the allocated region to the associated far memory node.
 * - swap-in amplification - penalties, when node swaps in span containing multiple objects, but only uses a small
 * number of these objects.
 * - spanset - a group of equal-sized spans.
 *
 * architecture
 * - compute nodes - single-process applications that want to use far memory.
 * - memory nodes - provide far memory for compute nodes.
 * - memory manager - tracks the liveness of compute nodes and memory nodes.
 *   - implemented as a replicated state machine.
 *   - it is assumed that it will not fail.
 *
 * integration
 * - exposes far memory to developers via application-level remotable pointers.
 *   - when application allocates a new object, Carbink rounds the object size up to the nearest size class and
 *     assigns a free object slot from an appropriate span.
 *   - after a compute node swaps in a far span, the node deallocates the far span.
 *   - filtering threads iterate through the objects in locally-resident spans and move those objects to different
 *     local spans.
 *   - object shuffling to create hot spans and cold spans. When local memory pressure is high, Carbink's eviction
 *     threads prefer to swap out spansets containing cold spans.
 *     - object hotness is tracked using GC-style read/write barriers. CLOCK algorithm is used to work with the
 *     hotness byte.
 *   - far memory is swapped into local memory at the granularity of a span.
 * - does not require custom hardware.
 *
 * reliability
 * - liveness of compute nodes and memory nodes is tracked via heartbeats.
 * - when a compute node fails, memory manager instructs memory nodes to deallocate.
 * - when a memory node fails, memory manager deregisters the node's regions from the global pool of memory.
 * - parity data is computed across all spans in the spanset. To reconstruct a span, a compute node merely has to
 * contact the single memory node which stores the span.
 * - RPCs involve software-level overheads, so one sided RMA is used.
 * - planned and unplanned failures
 *   - for planned, memory manager orchestrates
 * - degraded read - in case of failure, reads may be a bit slower because spans are reconstructed from parity data.
 */
pub fn carbink() -> Block {
    Block::Multiple(vec![
        subsection_header("Carbink: Fault-Tolerant Far Memory"),
        paragraph(vec![
            reference(
                "Carbink",
                Reference::for_publication(
                    "Carbink: Fault-tolerant Far Memory",
                    "Yang Zhou, Hassan Wassel, Sihang Liu, Jiaqi Gao, James Mickens, Minlan Yu, Chris Kennelly, Paul Jack Turner, David E Culler, Hank Levy, Amin Vahdat",
                    2022,
                    "Proceedings of the 16th USENIX Symposium on Operating Systems Design and Implementation",
                )
            ),
            " це система віддаленої памʼяті розроблена та протестована компанією Google в своїх центрах \
обробки даних. Ця реалізація фокусується на покращені відмовостійкості та зниженні рівня затримок при роботі з віддаленою \
памʼяттю.".into(),
        ]),
        paragraph("Система складається з декількох компонентів: memory nodes (вузли, що зберігають дані), compute nodes \
(програмне забезпечення, в яке інтегрується віддалена памʼять) та memory manager (менеджер памʼяті). Менеджер памʼяті \
керує розподілом фрагментів памʼяті по вузлах, що їх зберігають та перевіряє стан роботи цих вузлів. Важливим припущенням є \
те, що вважається що менеджер памʼяті завжди залишається в робочому стані. Мережевий звʼязок теж вважається стабільним."),
        paragraph("Для інтеграції у програмне забезпечення використовується бібліотека яка надає застосунку доступ до \
даних у віддаленій памʼяті через розумні показчики. При розміщенні даних у віддаленій памʼяті вони \
групуються у сторінки (spans)."),
        paragraph(vec![
            "Групування сторінок відбувається за їх розміром - такий самий підхід, який \
використовується у алокаторах памʼяті, наприклад у ".into(),
            reference("TCMalloc", Reference::for_publication(
                "Beyond malloc efficiency to fleet efficiency: a hugepage-aware memory allocator",
                "Andrew Hamilton Hunter, Chris Kennelly, Darryl Gove, Parthasarathy Ranganathan, Paul Jack Turner, Tipp James Moseley",
                2021,
                "15th USENIX Symposium on Operating Systems Design and Implementation (OSDI 21)"
            )),
            ". Окремий потік у фоновому режимі переміщує обʼєкти, доступ \
до яких відбувається частіше у спільні сторінки. Для того, щоб відслідковувати частоту доступу до обʼєктів, \
використовується підхід схожий на барʼєри запису та читання у прибиральниках сміття. Для роботи з байтом активності \
(hotness byte) використовується алгоритм ".into(),
            reference("CLOCK", Reference::for_website("Clock Algorithm, Second Chance List Algorithm, and Intro to I/O // CS162", "https://inst.eecs.berkeley.edu/~cs162/sp20/static/sections/section8-sol.pdf")),
            ". Також слід зазначити, що переміщення даних між локальною та \
віддаленою памʼяттю відбувається на рівні гранулярності сторінки (span). Коли системі потрібно звільнити памʼять, то \
переміщуються у першу чергу сторінки з нвйбільшою кількістю холодних обʼєктів.".into()
        ]),
        paragraph("Для забезпечення відмовостійкості, менеджер памʼяті постійно перевіряє стан інших вузлів (як тих, що \
зберігають дані, так і тих, на яких розміщене програмне забезпечення, в яке інтегрується віддалена памʼять) за допомгою \
healthcheck запитів. Якщо вузел обчислень (compute node) виходить з ладу, то вузлам що зберігають дані, надається команда \
на звільнення памʼяті. Якщо вузел зберігання даних (storage node) виходить з ладу, то дані відновляться з використанням \
частин парності (parity shards) які були розміщені на інших вузлах після кодування стиранням (кодування відбувається одразу \
на рівні декількох сторінок). На час відновлення даних та переміщення на новий вузел очікується більш високий рівень \
затримки операції читання. Також менеджер памʼяті підтримує планове обслуговування вузлів, коли дані завчасно \
переміщуються."),
        paragraph("Головним недоліком цієї реалізації віддаленої памʼяті є те, що програмний код не є публічно доступним, а \
також його привʼязаність до програмної та апартної інфраструктури, що використовується в компанії Google. Крім цього, ця реалізація \
не має механізмів оптимізації параметрів роботи з урахуванням закономірностей у доступі до даних програмним забезпеченням.")
    ])
}
