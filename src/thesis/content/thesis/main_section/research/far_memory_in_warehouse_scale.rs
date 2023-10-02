use crate::thesis::engine::{Block, paragraph};

/**
 * "Software-Defined Far Memory in Warehouse-Scale Computers"
 * see: https://storage.googleapis.com/pub-tools-public-publication-data/pdf/9bb06ab825a127bef4e33c488eaa659d6856225a.pdf
 * 
 * "maximize memory savings while meeting performance SLOs".
 * 
 * - nothing is stored remotely. This is all about compression and decompression with zswap.
 * - goal is to design a robust and effective control plane for latge-scale deployment of zswap (which is far memory).
 * - performance is treated as a first-class constraint.
 * - quality of the cold page identification algorithm impacts both memory savings and application impact.
 *   - system tries to find the lowest cold age threshold that satisfies the given performance constraints in order to maximize the
 *   memory savings under the defined SLO (in other words, system tries to mark as many pages as cold as possible while keeping the
 *   performance at a level defined by SLO).
 *   - performance SLO is based on the promotion rate.
 *     - target promotion rate is normalized by how "big" each job is.
 *     - SLO is keeping the promotion rate below P% of the application's working set size per minute.
 *       - working set size is defined as the total number of pages that are accessed within minimum cold age threshold.
 *       - the exact value of P depends on the performance difference between near memory and far memory.
 *         - empirically defined P to be 0.2%/min for their use-case to be optimal based on A/B-testing at scale.
 *   - promotion histogram is built to determine the lowest cold age threshold that meets the promotion rate SLO. Number of pages
 *   is also tracked to know what working set size is.
 *     - but this threshold is good for the past, not for the current workload. That's why K-th percentile is computed for the past
 *     when picking threshold for next minute.
 *     - ML-based Autotuner picks parameters for Node Agent which peeks Cold Age Threshold.
 * 
 * - zswap is triggered only when a host memory node runs out of memory and tries to compress pages until it makes enough room to avoid
 * out-of-memory situations.
 *   - the primary difference from existing mechanism is around when to compress pages, or when to migrate pages from near memory to
 *   far memory.
 *   - unlike zswap in the Linux kernel, this system identifies cold memory pages in the background and proactively compresses them,
 *   so that the extra free memory can be used to schedule more jobs to the machine.
 * 
 * terms
 *  - cold page - memory page that has not been accessed beyond a threshold of T seconds.
 *  - promotion rate - rate of accesses to cold memory pages.
 *    - equivalent to the number of unique pages in far memory that are accessed in a unit of time.
 * 
 * challenges:
 * - system has to accurately control its aggressiveness to minimize the impact on application performance (i.e. latency should be low).
 * - be resilient to the variation of cold memory behaviour accross different machines, clusters and jobs (i.e. should adapt to the 
 * environment).
 * 
 * integration
 * - zswap-based
 * 
 * cold page identification
 * - relies only on the working set size, promotion histogram and the cold page histogram.
 * - offline what-if analysis is possible.
 * - stats are exported in 5-minute perioud aggregations over telemetry infrastructure.
 * 
 * autotuning
 *  - uses machine learning to optimize the control plane based on the fleet-wide behaviour.
 *  - fast far memory model estimating behaviour under different configurations.
 *  - design space exploration guided by machine learning algorithm called Gaussian Process (GP) Bandit (black-box optimization).
 *  - improves the efficiency of the system by an additional 30% relative to heuristic-based approaches.
 *  - K-th percentile and S (time until zswap starts) are tunable hyperparameters for the control plane.
 *  - optimization pipeline steps:
 *    - run GP bandit over the existing observations and obtrain parameter configurations to be explored.
 *    - run the far memory model with a one week trace from the entire WSC and estimate the size of cold memory and promotion range.
 *    - add new observations to the pool and go back to step 1 until the maximum number of iterations is reached.
 *    - the best configuration is deployed.
 */
pub fn far_memory_in_warehouse_scale() -> Block {
    Block::Multiple(vec![
        Block::SubsectionHeader("Software-Defined Far Memory in Warehouse-Scale Computers".to_owned()),
        Block::Placeholder(
            Box::new(paragraph("Компанія Google розробила та в тестовому режимі інтегрувала систему віддаленої памʼяті в своїх центрах обробки даних. Наскільки відомо на момент проведення дослідження існуючих реалізацій для цього курсового проекту, це єдиний великий оператор центрів обробки даних, який спроектував та застосував систему віддаленої памʼяті на великих масштабах та обладнанні яке виконує програмне забезпечення, що обробляє запити від користувачів (іншими словами, на справжньому навантаженні, а не у тестовому середовищі).".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(paragraph("В науково-дослідній роботі, опублікованій Google, зазначається що система для своєї роботи використовує функціонал ядра операційної системи Linux під назвою zswap. Перевагою такого рішення є те, що zswap це перевірена часом технологія, яка надає можливість компресії сторінок памʼяті. Інтеграція з цим функціоналом дозволяє переносити частини памʼяті на віддалені вузли без використання додаткового апаратного забезпечення.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(paragraph("Система відслідковує час останнього доступу до сторінок памʼяті для визначення “холодних” сторінок, які було б ефективно перенести у віддалену памʼять. Також окремий компонент системи відслідковує сторінки, які потрібно перенести з віддаленої памʼяті у локальну. Обидва з цих компонентів контролюються встановленим на кожен сервер агентом віддаленої памʼяті. Ця програма збирає статистику використання памʼяті та задає параметри роботи для системи.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(paragraph("В публікації зазначено, що режими доступу до памʼяті постійно змінюються. Наприклад, зібрана статистика показує що кількість холодних сторінок памʼяті варіюʼться від 1% до 61% в залежності від часу дня, програмного забезпечення що розгорнуто на серверному обладнанні, запитів від користувачів та конкретного серверу. Це створює необхідність мати окремий компонент автоматичного налаштування системи. Цей компонент отримує статистику зібрану агентами на обчислювальних вузлах та за допомогою моделей машинного навчання обирає нові значення параметрів для компонентів що керують переміщенням сторінок памяʼті.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(paragraph("Недоліком системи є те, що її програмний код не є публічно доступним, а окремі компоненти є специфічними для середовища, що використовується в компанії Google.".to_owned())),
            "improve this text".to_owned(),
        ),
    ])
}