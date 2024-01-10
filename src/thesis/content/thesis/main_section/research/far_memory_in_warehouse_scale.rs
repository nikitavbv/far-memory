use crate::thesis::engine::{Block, paragraph, subsection_header, Reference, reference};

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
        subsection_header("Software-Defined Far Memory in Warehouse-Scale Computers"),
        paragraph(vec![
            reference("Ця", Reference::for_publication(
                "Software-defined far memory in warehouse-scale computers".to_owned(),
                "Andres Lagar-Cavilla, Junwhan Ahn, Suleiman Souhlal, Neha Agarwal, Radoslaw Burny, Shakeel Butt, Jichuan Chang, Ashwin Chaugule, Nan Deng, Junaid Shahid, Greg Thelen, Kamil Adam Yurtsever, Yu Zhao, Parthasarathy Ranganathan".to_owned(),
                2019, "International Conference on Architectural Support for Programming Languages and Operating Systems".to_owned()
            )),
            " реалізація віддаленої памʼяті зберігає дані не на віддалених вузлах, а у локальній памʼяті, але у стиснутому стані. Для цього, реалізація спирається \
на функціонал ".into(),
            reference("zswap", Reference::for_website("zswap // ArchWiki", "https://wiki.archlinux.org/title/zswap")),
            " у операційній системі Linux.".into(),
        ]),
        paragraph("Найбільший ефект на швидкодію програмного забезпечення та кількість звільненої памʼяті має алгоритм ідентифікації холодних сторінок (тобто сторінок \
памʼяті з низькою частотою доступу). Ціллю алгоритму є пошук такого рівню відносно якого сторінки вважаються холодними, щоб максимізувати кількість сторінок у віддаленій \
памʼяті при цьому задовольняючи вимоги програмного забезпечення щодо швидкодії. Цей рівень виражається у частці від обʼєму памʼяті, доступ до якої програма виконує за \
хвилину. Для встановлення значення цього рівня будується гістограма за попередньо зібраною статистикою та обирається той рівень, який задовлльняє вимогам по швидкодії."),
        paragraph("Виявлення сторінок які задовольняють критерію відбувається у фоновому режимі, після чого zswap виконує стиснення цих даних для звільнення памʼяті."),
        paragraph(vec![
            "Окремий компонент збирає статистику з усіх вузлів у кластері та за допомогою алгоритмів машинного навчання оптимізує значення гіперпараметрів. Це дозволяє \
оптимізувати агресивність алгоритму перенесення даних у віддалену памʼять з урахуванням різниці у закономірностях доступу до памʼяті між обчислювальними вузлами у кластері, \
різними датацентрами та різним програмним забезпеченням. Дані збираються за допомогою існуючої інфраструктури збору телеметрії. Для оптимізації використовується алгоритм \
машинного навчання ".into(),
            reference("Gaussian Process (GP) Bandit", Reference::for_website("Gaussian Process (GP) Bandits", "https://acsweb.ucsd.edu/~shshekha/GPBandits.html")),
            ", який є оптимізацією чорного ящику (".into(),
            reference("black-box optimization", Reference::for_book("Andrew R. Conn, Katya Scheinberg, Luis N. Vicente", "Introduction to Derivative-Free Optimization", "University City", 2009, 289)),
            "). У роботі зазначається, що це підвищує ефективність системи \
на 30% у порівнянні з евристичними підходами. Алгоритм генерує декілька можливих конфігурацій параметрів системи, які оцінюються за допомогою симуляції роботи за статистикою, \
що була зібрана протягом тижня. Найкраща конфігурація (за показником звільненої памʼяті) передається для використання на всьому кластері.".into()
        ]),
        paragraph("Недоліками цієї реалізації є те, що вона використовує локальну памʼять для збереження даних, а не памʼять віддалених вузлів. Крім цього, код реалізації є \
закритим, недоступним для використання ззовні та привʼязаним до інфраструктури що використовується у компанії Google."),
    ])
}
