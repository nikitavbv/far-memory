use crate::engine::Block;

/**
 * "Software-Defined Far Memory in Warehouse-Scale Computers"
 * see: https://storage.googleapis.com/pub-tools-public-publication-data/pdf/9bb06ab825a127bef4e33c488eaa659d6856225a.pdf
 * (taking notes on page 5)
 * 
 * - goal is to design a robust and effective control plane for latge-scale deployment of zswap (which is far memory).
 * - performance is treated as a first-class constraint.
 * - quality of the cold page identification algorithm impacts both memory savings and application impact.
 *   - system tries to find the lowest cold age threshold that satisfies the given performance constraints in order to maximize the
 *   memory savings under the defined SLO (in other words, system tries to mark as many pages as cold as possible while keeping the
 *   performance at a level defined by SLO).
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
 * 
 * challenges:
 * - system has to accurately control its aggressiveness to minimize the impact on application performance (i.e. latency should be low).
 * - be resilient to the variation of cold memory behaviour accross different machines, clusters and jobs (i.e. should adapt to the 
 * environment).
 * 
 * autotuning
 *  - uses machine learning to optimize the control plane based on the fleet-wide behaviour.
 *  - fast far memory model estimating behaviour under different configurations.
 *  - design space exploration guided by machine learning algorithm called Gaussian Process (GP) Bandit.
 *  - improves the efficiency of the system by an additional 30% relative to heuristic-based approaches.
 */
pub fn far_memory_in_warehouse_scale() -> Block {
    Block::Multiple(vec![
        Block::SubsectionHeader("Software-Defined Far Memory in Warehouse-Scale Computers".to_owned()),
        Block::Placeholder(
            Box::new(Block::Paragraph("Компанія Google розробила та в тестовому режимі інтегрувала систему віддаленої памʼяті в своїх центрах обробки даних. Наскільки відомо на момент проведення дослідження існуючих реалізацій для цього курсового проекту, це єдиний великий оператор центрів обробки даних, який спроектував та застосував систему віддаленої памʼяті на великих масштабах та обладнанні яке виконує програмне забезпечення, що обробляє запити від користувачів (іншими словами, на справжньому навантаженні, а не у тестовому середовищі).".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("В науково-дослідній роботі, опублікованій Google, зазначається що система для своєї роботи використовує функціонал ядра операційної системи Linux під назвою zswap. Перевагою такого рішення є те, що zswap це перевірена часом технологія, яка надає можливість компресії сторінок памʼяті. Інтеграція з цим функціоналом дозволяє переносити частини памʼяті на віддалені вузли без використання додаткового апаратного забезпечення.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Система відслідковує час останнього доступу до сторінок памʼяті для визначення “холодних” сторінок, які було б ефективно перенести у віддалену памʼять. Також окремий компонент системи відслідковує сторінки, які потрібно перенести з віддаленої памʼяті у локальну. Обидва з цих компонентів контролюються встановленим на кожен сервер агентом віддаленої памʼяті. Ця програма збирає статистику використання памʼяті та задає параметри роботи для системи.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("В публікації зазначено, що режими доступу до памʼяті постійно змінюються. Наприклад, зібрана статистика показує що кількість холодних сторінок памʼяті варіюʼться від 1% до 61% в залежності від часу дня, програмного забезпечення що розгорнуто на серверному обладнанні, запитів від користувачів та конкретного серверу. Це створює необхідність мати окремий компонент автоматичного налаштування системи. Цей компонент отримує статистику зібрану агентами на обчислювальних вузлах та за допомогою моделей машинного навчання обирає нові значення параметрів для компонентів що керують переміщенням сторінок памяʼті.".to_owned())),
            "improve this text".to_owned(),
        ),
        Block::Placeholder(
            Box::new(Block::Paragraph("Недоліком системи є те, що її програмний код не є публічно доступним, а окремі компоненти є специфічними для середовища, що використовується в компанії Google.".to_owned())),
            "improve this text".to_owned(),
        ),
    ])
}