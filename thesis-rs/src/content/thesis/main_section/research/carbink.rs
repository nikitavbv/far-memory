use crate::engine::Block;

/**
 * "Carbink: Fault-tolerant Far Memory"
 * see: https://www.usenix.org/system/files/osdi22-zhou-yang.pdf
 * (currently on page 11)
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
    ])
}
