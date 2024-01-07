use crate::thesis::engine::{Block, paragraph, subsection_header, Reference, reference};

/**
 * "Hydra : Resilient and Highly Available Remote Memory"
 * see https://www.usenix.org/system/files/fast22-lee.pdf
 *
 * TL;DR: Using CodingSets and RDMA hardware to achieve low latency and high-availability.
 *
 * cons:
 * - it uses RDMA (requires NIC that supports it).
 *
 * challenges:
 * - expanded failure domains.
 * - tail at scale.
 *
 * existing solutions address by:
 * - local disk backup (access latency is too high).
 * - remote in-memory replication (cost is too high).
 * - remote in-memory erasure coding.
 * - compression.
 *
 * Erasure coding works better for larger chunks for data.
 * CodingSets (inspired by copysets) - coding group placement algorithm for erasure-coded data.
 *
 * components:
 * - resilience manager - coordinates erasure-coded resilience operations during remote read/write.
 *   - implemented as a loadable kernel moudle.
 * - resource monitor - handles the memory management in a remote machine.
 *   - implemented as a user-space program.
 *
 * improving reliability:
 * - different modes of erasure coding.
 * - when a remote machine becomes unreachable, ongoing I/O operations are resent to other available machines.
 *
 * improving latency:
 * - during remote write, resilience manager applies erasure coding to splits, encodes them using Reed-Solomon codes
 * and decreases latency by avoiding the batch waiting time.
 * - resilience manager sends the data splits first, then encodes and sens the parity splits asynchronously.
 * - a page can be decoded as soon as any k splits arrive out of k + delta.
 * - smaller RDMA messages lead to lower latency.
 */
pub fn hydra() -> Block {
    Block::Multiple(vec![
        subsection_header("Hydra: Resilient and Highly Available Remote Memory"),
        paragraph(vec![
            reference("Hydra", Reference::for_publication(
                "Hydra: Resilient and Highly Available Remote Memory".to_owned(),
                "Youngmoon Lee, Hasan Al Maruf, Mosharaf Chowdhury, Asaf Cidon, Kang G. Shin".to_owned(),
                2022,
                "20th USENIX Conference on File and Storage Technologies (FAST 22)".to_owned(),
            )),
            " це реалізація віддаленої памʼяті, яка використовує RDMA для передачі даних між вузлами, забезпечує відмовостійкість та \
                вирішує задачу інтеграції у програмне забезпечення.".into(),
        ]),
        paragraph("Hydra інтегрується у програмне забезпечення через використання механізму підкачки у операційній системі Linux. Компонент \
системи (resource monitor) що працює у просторі користувача надає віртуальний блоковий пристрій, на якому розміщується розділ підкачки памʼяті. Це \
надає можливість інтегруватись у нове та існуюче програмне забезпечення без змін у код."),
        paragraph(vec![
            "Для забезпечення відмовостійкості, у цій роботі пропонується підхід під назнвою CodingSets. Під час перенесення даних у памʼять \
інших вузлів, виконується кодування кодом ".into(),
            reference("Ріда-Соломона", Reference::for_website("An introduction to Reed-Solomon codes: principles, architecture and implementation".to_owned(), "https://www.cs.cmu.edu/~guyb/realworld/reedsolomon/reed_solomon_codes.html".to_owned())),
            ", що дає декілька частин даних (data splits) та парності (parity splits). Підхід CodingSets \
полягає у тому, щоб оптимально розподілити частини між вузлами таким чином, щоб не тільки розподілити навантаження, а й максимально знизити ймовірність \
втрати даних у разі одночасного виходу з ладу декількох вузлів. Операції кодування та розподілення частин виконуються компонентом resilience manager \
який реалізований як модуль ядра операційної системи. Коли один з вузлів виходить з ладу, операції читання та запису перенаправляються на інші вузли, \
доки відбувається відновлення даних на новий вузел.".into(),
        ]),
        paragraph("Низький ріень затримки доступу до даних у віддаленій памʼяті забезпечується використанням RDMA, в тому числі тим фактом що малі \
повідомлення (як результат кодування) швидко передаються. Крім цього, під час перенесення даних з віддаленої памʼяті у локальну системі достатньо отримати \
частини даних лише з частки вузлів (число залежить від параметрів кодування стиранням, які були обрані користувачем). Для зниження затримки під час \
запису даних, ця реалізація віддаленої памʼяті відправляє дані парності асинхронно, очікуючи на успішний запис лише основної частини даних."),
        paragraph("Головним недоліком цієї реалізації віддаленої памʼяті є залежність від RDMA, що вимагає спеціального апартного забезпечення (мережевих \
карт, що її підтримують). Оскільки RDMA не є розповсюдженим у сучасних центрах обробки даних (за винятком спеціалізованих HPC кластерів), то можливість \
використання Hydra на практиці є обмеженим."),
    ])
}
