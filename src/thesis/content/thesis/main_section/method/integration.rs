use crate::thesis::engine::{Block, subsection_header, paragraph, ImageBlock, reference, Reference};

pub fn integration() -> Block {
    Block::Multiple(vec![
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
        Block::Image(ImageBlock::new("./images/integration.png".to_owned(), "Приклад використання бібліотеки клієнту віддаленої памʼяті".to_owned())),
        paragraph("При створенні розумного показчика (FarMemory<T>), він привʼязується до одного з проміжків памʼяті (span). В залежності від розміру обʼєкта, створюється або новий проміжок з необхідним розміром, або \
обʼєкт разом з іншими обʼєктами розміщується в межах одного проміжку, з різними значеннями зміщення відносно початку проміжку. Розумний показчик FarMemory<T> зберігає або ідентифікатор проміжку (span) якщо це великий \
обʼєкт, або ідентифікатор обʼєкта якщо це малий обʼєкт. У клієнті віддаленої памʼяті (FarMemoryClient) зберігається таблиця, для ключем є ідентифікатор обʼєкту, а значенням - ідентифікатор проміжку та зсув у ньому. \
Необхідність цієї таблиці зумовлена тим, що вона дозволяє переміщувати обʼєкти між проміжками у разі необхідності. Це потрібно наприклад для розміщення обʼєктів доступ до яких зазвичай відбувається одночасно у межах \
одного проміжку для зниження кількості операцій переміщення проміжків між пристроями памʼяті. Це також робить можливим реалізацію дефрагментації (compaction) яка знижує закальну кількість проміжків, що використається \
програмним забезпеченням."),
        paragraph("Коли розумний показчик отримає запит на доступ до даних (у мові програмування Rust це реалізується операцією Deref), то він звертається до клієнту віддаленої памʼяті для отримання адреси проміжку \
памʼяті в оперативній памʼяті. Якщо потрібний проміжок в цей час вже знаходиться в оперативній памʼяті, то клієнт просто повертає його адресу (*mut u8). Якщо потрібного проміжку в памʼяті немає, то клієнт переміщує \
його з памʼяті пристроїв зберігання (бекенду) у локальну памʼять, після чого повертається та адреса, де було розміщено цей проміжок памʼяті. Прикладне програмне забезпечення після цього працює з даними що розміщені \
у цій адресі так само, як працювало б з будь-якими іншими даними у оперативній памʼяті."),
        paragraph("Суть роботи клієнту віддаленої памʼяті полягає в тому, що проміжки памʼяті (spans) які не використовуються у певний момент часу програмним забезпеченням можуть бути переміщені частково або \
повністю у більш повільну памʼять віддаленого пристрою (бекенду). Це створює необхідність відслідковувати які обʼєкти та проміжки використовуються програмним забезпеченням у певний момент часу, а які - ні. \
Для цього можна використати наступний підхід: розумний показчик FarMemory<T> після операції розіменування (dereferencing - функція deref у Rust) повертає інший розумний показчик (FarMemoryLocal<T>), який у \
результаті розміменування повертає вже сам обʼєкт T. Цей другий розумний показчик потрібен для контролю використання даних з віддаленої памʼяті. Мова програмування Rust та механізм borrow checking у ній \
гарантує що посилання на T (&T) не буде більше використовуватись після того, як розумний показчик FarMemoryLocal<T> вийшов з зони видимості. Це дає можливість використовувати метод підрахунку посилань для \
проміжків памʼяті (spans). При розіменовуванні першого показчика (FarMemory<T>) лічильник посилань для проміжку памʼяті збільшується. При виходу з видимості другого розумного показчика (FarMemoryLocal<T>) - \
лічильник посилань зменшується. Клієнт віддаленої памʼяті переміщує проміжки памʼяті для зберігання у бекенд тільки коли лічильник посилань дорівнює нулю."),
        paragraph("При переміщенні обʼєкту за показчик у віддаленій памʼяті виникає проблема його перетворення на послідовність байт для представлення у памʼяті та передачі мережею чи запису на диск. Найбільш \
простим методом зробити це є взяти адресу у якій розміщено обʼєкт T та перемістити у віддалену памʼять з тієї адреси стільки байт, скільки є розміром типу T. Цей підхід використовується у FarMemory<T> і \
для більшості сценаріїв використання цього методу достатньо. Але є структури даних для яких використання такого підходу не є оптимальним. Це, наприклад, структури які мають показчики на інші структури у \
памʼяті. Якщо перекопіювати дані з памʼяті де знаходиться ця структура даних, то скопіюється не значення поля, а показчик на структуру даних, яка там знаходиться. Для користувача зазвичай необхідно щоб \
обʼєкт був переміщений у віддалену памʼять повністю, з усіма вкладеними полями, так як це дозволяє звільнити максимальну кількість памʼяті. Вирішенням цієї проблеми є створення розумного показчика (\
FarMemorySerialized<T>) який не копіює дані з памʼяті, а використовує механізм серіалізації. Для цього на тип T накладається вимога реалізувати ознаки (trait) Serialize та Deserialize з бібліотеки serde. \
Під час роботи десеріалізація відбувається у момент доступу до даних за розумним показчиком. Негативним наслідком цього підходу є додаткове використання ресурсів, яке викликане цим. Для операцій серіалізації \
та десеріалізації використовується бібліотека bincode, так как для більшості типів вона копіює дані з памʼяті без змін, уникаючи зайвих операцій. Іншими словами, цей підхід максимально близький до рекурсивного \
обходу структури даних."),
        paragraph("Можливість розміщувати обʼєкти у віддаленій памʼяті за допомогою розумних показчиків може бути достатньо для багатьох випадків, але у прикладному програмному забезпеченні часто використовуються \
так звані \"великі буфери\" що створюються програмою за допомогою алокатора, та використовуються як великий послідновний обсяг памʼяті для збереження різних даних в оптимальному вигляді для конкретної задачі. \
При цьому такий буфер надає можливість читання та запису байтів з довільного зсуву та довжини. Для того, щоб зробити роботу з таким буфером більш оптимальною у випадку віддаленої памʼяті, у бібліотеку клієнту \
віддаленої памʼяті додається FarMemoryBuffer. Він має такий самий інтерфейс та зберігає дані розбиваючи їх на необхідну кількість проміжків (spans). Кожен проміжок має однаковий розмір вказаний користувачем \
або 2 мегабайти за замовчуванням. Таким чином, лише частина буфера знаходиться у локальній памʼяті (ті, до яких відбувається доступ), тоді як всі інші дані - у віддаленій. Це дозволяє знизити використання памʼяті \
та створювати буфери для зберігання даних розмір яких перевищує кількість оперативної памʼяті обчислювального вузла, що є дуже зручним у відповідних сценаріях використання. Цей тип можна розглядати як аналогічний \
звичайній області памʼяті отриманій від алокатора та використовувати розробниками як основу при реалізації власних структур даних, адаптованих під використання віддаленої памʼяті."),
        paragraph("Крім цього, FarMemoryBuffer як і деякі інші реалізації структур даних вирішують одну з проблем використання розумних показчиків: у випадку великого обʼєкта вони надають можливість розміщувати \
у локальній оперативній памʼяті лише його частину. Ця особливість надається завдяки тому, що доступ до частини даних надається через визначений інтерфейс, де є контекст до якої саме частини даних відбувається \
доступ, на відміну від показчиків, коли прикладному програмному забезпеченню надається адреса у памʼяті та довжина даних і клієнту віддаленої памʼяті з цього моменту невідомо як саме програма працює з даними \
у цій ділянці оперативної памʼяті."),
        paragraph("Розвиваючи цю ідею далі, у бібліотеку клієнту віддаленої памʼяті було додано реалізацію вектора (Vec<T> у Rust) адаптованого для роботи з віддаленою памʼяттю: FarMemoryVec<T>. Він передбачає \
доступ як до частини даних, так і до усіх даних одночасно перетворивши їх на звичайний вектор (Vec<T>) за допомогою розумного показчика. Особливістю цієї реалізації є те, що вона використовує один проміжок \
памʼяті (span). В деяких сценаріях використання (наприклад великий масив, доступ до якого буде відбуватися завжди у вигляді доступу до конкретного елементу або невеликої частини масиву), більш оптимальним \
є використання вектору який розбиває дані на багато невеликих проміжків, тому для таких цілей клієнтом надається FarMemoryBufferedVec<T>."),
        paragraph(vec![
            "Альтернативним підходом для інтеграції віддаленої памʼяті у тих випадках, коли будь-які зміни в код програмного забезпечення не є бажаними є інтеграція за допомогою механізму підкачки у \
операціній системі Linux. Як було зазначено раніше, у цього підходу є недоліки у вигляді меншої гнучкості та додаткового контексту, який можна було б використати для підвищення швидкодії віддаленої памʼяті. \
Для використання віддаленої памʼяті для підкачки у Linux використовується підхід подібний тому, що використовують деякі існуючі реалізації методів надання віддаленої памʼяті, а саме реалізацію клієнтом віддаленої \
памʼяті ".into(),
            reference("віртуального блокового пристрою", Reference::for_website("Block Device Driver // The Linux Kernel documentation".to_owned(), "https://linux-kernel-labs.github.io/refs/heads/master/labs/block_device_drivers.html".to_owned())),
            ". Користувач налаштовує розміщення розділу підкачки у операційній системі Linux на цьому пристрої. В результаті, сторінки памʼяті які операційна система вважає рідко \
вживаними переміщуються з основної памʼяті у розділ підкачки (тобто, в цьому випадку у віддалену памʼять) у випадку коли використання локальної оперативної памʼяті досягає високого рівня.".into(),
        ]),
        // tell how library and service running on the end node would be working more specifically. Tell how spans are swapped in and out here. Tell about remotable pointers, buffers, optimized data structures and streaming.
        // tell about ref-counting and identification which spans are not in use. explain how memory limits work. explain how user is supposed to use the swap file and what is done to prevent recursive swap (idk if that is
        // right name for that). tell about traces as a way to monitor the system (if that is a good topic to explain).
        // explained objects serialization.
        // explain why NBD is used for running on the end node. tell that options are a library or a service running on the end node.
        // Tell how span size is being chosen and what effect it has.
    ])
}
