use crate::thesis::engine::{Block, subsection_header, paragraph};

pub fn performance() -> Block {
    Block::Multiple(vec![
        subsection_header("Забезпечення швидкодії віддаленої памʼяті"),
        paragraph("Головним недоліком використання віддаленої памʼяті є негативний ефект на швидкодію програмного забезпечення, у яке вона інтегрована. Погіршення швидкодії виникає через більш \
повільний доступ до памʼяті який зумовлений в першу чергу використанням пристроїв зберігання (таких як памʼять інших вузлів, доступ до якої виконується за мережею, дисків, та ін.) затримка \
(latency) та пропускна здатність (throughput) яких є значно більшою за такі параметри для оперативної памʼяті. Крім цього, під час доступу до проміжку даних у віддаленій памʼяті виконуються \
додаткові перевірки та дії, які не є потрібними при роботі зі звичайною памʼяттю."),
        paragraph("Неможливо зробити доступ до віддаленої памʼяті таким саме швидким як доступ до оперативної памʼяті (у умовах, коли пристрій зберігання є повільним). Але має сенс мінімізувати \
затримку доступу до даних до того рівня, який є прийнятним для використання на практиці. Існує баланс між тим наскільки активно віддалена памʼять використовується програмним забезпеченням (\
скільки даних та якого типу в ній зберігається) та негативним ефектом на швидкодію програмного забезпечення. Доцільність використання віддаленої памʼяті та параметрів її роботи є відповідальністю \
розробника прикладного програмного забезпечення. Конфігурація віддаленої памʼяті обирається розробником базуючись на вимогах щодо швидкодії програмного забезпечення, його особливостях роботи \
з памʼяттю та характеристках апаратного забезпечення (наприклад, швидкість мережі)."),
        paragraph("Як було зазначено раніше, кожен проміжок (span) у віддаленій памʼяті може в певний момент часу знаходитись у локальній памʼяті чи памʼяті віддаленого вузла \
(бекенду). В той час як доступ до даних у локальній памʼяті майже не відрізняється від часу доступу до даних у оперативній памʼяті (зазвичай декілька мікросекунд), час доступу \
до даних у памʼяті віддалених вузлів є значно більшим (десятки-сотні міллісекунд). Коли програмне забезпечення запитує дані які знаходяться у віддаленій памʼяті, то доки \
вони не будуть переміщенні у локальну памʼять, виконання програми не можна продовжувати. Це блокування є головною причиною негативного впливу на швидкодію. Для зниження \
часу доступу до даних, кількість а також довжину таких блокувань потрібно зменшувати."),
        paragraph("На швидкість переміщення даних між памʼяттю віддаленого вузла та локальною памʼяттю в першу чергу впливає швидкість переміщення даних мережею. Через це \
важливо ефективно використовувати ресурси мережевого обладнання. Для передачі даних було розроблено легкий бінарний протокол, який працює поверх TCP. В сучасному програмному \
забезпеченні для передачі даних часто використовуються протоколи віддаленого визову процедур (remote procedure call), такі як gRPC чи HTTP запити. Використання таких \
протоколів не є оптимальним для клієнту віддаленої памʼяті, оскільки їх реалізації покладаються на складні аглоритми серіалізації запитів, що виконують зайві копіювання \
та перетворення даних. Розробка легкого протоколу поверх TCP дозволяє уникнути цих недоліків високорівневих протоколів. TCP був обраний так як цей протокол на відміну від UDP \
забезпечує надійність та реалізовує необхідні механізми для максимізації ефективності використання каналу передачі даних. QUIC як альтернатива TCP в цій роботі не \
розглядається оскільки реалізації з використанням TCP вже достатньо для того, щоб швидкість передачі даних була близькою до пропускної здатності мережі, через що немає \
причин ускладнювати систему використанням QUIC."),
        paragraph("Для того, щоб знизити затримку (latency) передачі даних по TCP, клієнт віддаленої памʼяті не використовує алгоритм Нейгла (Naggle algorithm). Його \
задачею є підвищення швидкодії мережі через уникання невеликих пакетів даних під час передачі. Додаткове очіквання наступних пакетів не має сенсу, оскільки клієнт віддаленої \
памʼяті пише увесь запит за раз і навіть якщо він невеликий (наприклад, запит на отримання проміжку памʼяті), клієнт очікує відповідь одразу після запису запиту. Алгоритм \
Нейгла відключено через використання опції nodelay під час налаштування TCP сокету."),
        paragraph("Для того, щоб знизити час обробки при декількох послідовних запитах (наприклад, переміщення декількох проміжків одночасно), клієнт віддаленої памʼяті \
може обʼєднувати декілька запитів у один (batching) для більш ефективної обробки. Це дозволяє уникнути завершення попереднього запиту для надсилання наступного."),
        paragraph("В програмному забезпеченні, швидкодія якого залежить від швидкодії мережі для зниження затримки часто використовується підхід реалізації роботи з мережею \
у просторі користувача (userspace networking або kernel bypass networking). В тому числі цей підхід використовується у реалізації віддаленої памʼяті AIFM. В цій роботі \
kernel bypass networking не використовується, тому що зниження затримки (latency) передачі пакетів незначно вплине на час доступу до даних, тому що під час переміщення \
великих проміжків памʼяті обмежуючим фактором в першу чергу є пропускна здатність мережі (throughput). Використання kernel bypass networking також привʼяже реалізацію \
до конкретних моделей мережевих карт, що суперечить одній з вимог до системи. Отже, використання цього підходу та теоретично відносно невелике покращення швидкодії не є \
того вартим."),
        paragraph("Слід зазначити, що сучасне мережеве обладнання зазвичай підтримує одночасну передачу і отримання даних на максимальній швидкості, тобто підтримує \
двусторонню (duplex) передачу. Наприклад, стандартом 10 Gigabit Ethernet це є єдиним режимом зʼєднання який визначено, тобто програмне забезпечення може одночасно \
передавати та отримувати дані на швидкості 10 Гігабіт у секунду. У цій роботі, ця можливість апартного забезпечення використовується клієнтом віддаленої памʼяті для \
одночасного переміщення сторінок у памʼять віддалених вузлів та з неї. Це дозволяє максимально ефективно використовувати канал зʼєднання."),
        paragraph("Для того, щоб зменшити кількість зайвих перетворень та копіювань даних, дані проміжків памʼяті передаються мережею окремо від тіла запиту чи відповіді. \
Тіло запита чи відповіді містить поле, яке зберігає довжину проміжку памʼяті. Це прискорює серіалізацію запитів, оскільки тіло запиту стає значно меншим. Зайве копіювання \
(яке зазвичай займає порівняно великий проміжок часу) уникається, так як читання даних проміжку памʼяті з TCP сокету виконується напряму у ту адресу оперативної памʼяті \
де дані після цього будуть зберігатися. Для серіалізації запитів обрано формат bincode, так як це одна з найбільш швидких реалізацій серіалізацій у мові програмування \
Rust."),
        paragraph("Часто використання стиснення даних дозволяє знизити час передачі даних по мережі за рахунок використання додаткових ресурсів процесору для зменшення \
кількості даних, які потрібно передати. Це є ефективним при обмежених ресурсах пропускної здатності мережі. Однак, швидкість обробки найбільш швидких сучасних алгоритмів \
стиснення даних (таких як zstandard, snappy чи lz4) на сучасному обладнанні становить близько 800 Мегабайт у секунду (6,4 Гбіт у секунду), що у більшості випадків є \
меншим за пропускну здатність мережі (у центрах обробки даних пропускна здатність мережі між серверами зазвичай становить 10 Гбіт/сек та вище). Це робить використання \
стиснення даних недоцільним. Через це за замовчуванням клієнт віддаленої памʼяті не використовує стиснення, але залишає можливість його опціально увімкнути користувачу \
(використовується алгоритм lz4)."),
        paragraph("З такої самої причини (повільна обробка), за замовчуванням шифрування даних не виконується."),
        paragraph("Клієнт віддаленої памʼяті перевіряє рівень вільної локальної памʼяті під час переміщення проміжків з памʼяті віддалених вузлів. Можливо що для переміщення \
даних потрібно додатково звільнити певний обсяг локальної памʼяті через переміщення проміжків у зворотньому напрямку для зниження використання локальної памʼяті. Розміри \
проміжків памʼяті можуть перевищувати розмір памʼяті, яку потрібно звільнити. Це робить переміщення таких проміжків неефективним: переміщується більше даних ніж потрібно. \
Це можна було б вирішити комбінуванням невеликих проміжків памʼяті, які б в сумі були близькими до того обсягу памʼяті, який потрібно звільнити. Але це теж не є ефективним: \
це вимагає вирішення задачі рюкзака, що потребує значних обчислень, при цьому вільних (тобто тих, які не використовуються програмою та не будуть використані у найближчий \
час) невеликих проміжків може не бути. Через це, в цій роботі реалізовано часткове переміщення проміжків памʼяті: дані можуть бути розділені між локальною памʼяттю та \
памʼяттю віддалених вузлів у будь-якій пропорції, яка може змінюватись за необхідністю. Це дозволяє переміщувати у віддалену памʼять рівно стільки даних, скільки потрібно \
щоб звільнити локальну памʼять до достатнього рівня. У локальній памʼяті залишається частина з початку проміжку, у віддаленій - з кінця, це робить повторні операції \
часткового переміщення більш дешевими за рахунок того, що дані не потрібно копіювати або зсувати. В більшості випадків достатньо виклику алокатора, який зменшує чи збільшує \
розмір цієї ділянки памʼяті. Зберігання частини даних також залишає можливість обмежувати час доступу до великого проміжку памʼяті у гіршому випадку (частина даних що вже є \
в локальній памʼяті зменшує кількість даних що потрібно перемістити та відповідний час на це). Іншими словами, переміщення частин великої кількості проміжків замість \
повного переміщення невеликої кількості проміжків дозволяє знизити час доступу у гіршому випадку (tail latency) за рахунок більшої кількості опреацій переміщення проміжків."),
        paragraph("Як було зазначено раніше, невеликі обʼєкти вимагають розташування декількох обʼєктів у межах одного проміжку памʼяті, так як створення великої кількості \
малих проміжків підвищує витрати ресурсів необхідних для керування віддаленою памʼяттю. При цьому виникає проблема, схожа на ту, з якою стикаються алокатори памʼяті: \
фрагментація памʼяті призводить до неефективного використання ресурсів. У випадку віддаленої памʼяті це не тільки призводить до зайвого використання памʼяті, а й збільшує \
кількість операцій доступу до проміжків памʼяті та їх переміщення, що має негативний ефект на швидкодію. Для вирішення цієї проблеми у цій роботі використовується підхід \
класів розміру (size classes), подібний до того, що використовується у Carbink чи TCMalloc. Цей підхід полягає у тому, що кожен з проміжків зберігає обʼєкти одного розміру \
(можливі розміри задані завчасно, при додаванні обʼєкту його розмір округляється до найближчого класу розміру). Коли обʼєкт прибирається з віддаленої памʼяті, то на його \
місці можна розмістити обʼєкт такого ж розміру. В результаті, фрагментація знижується і переміщення обʼєктів між локальною та віддаленою памʼяттю становиться більш \
ефективним."),
        paragraph("Незважаючи на важливість швидкої передачі проміжків по мережі та зниження фрагментації, ключем до забезпечення низької затримки є розміщення проміжків \
памʼяті таким чином, щоб потрібні дані в найбільшій кількості випадків знаходились у локальній памʼяті, доступ до якої є швидким. Як зазначалось раніше, важливим є зменшення \
часу, коли програмне забезпечення блокується очікуючи даних по мережі. Одним зі способів як цього досягти є створення фонового потоку виконання, який виконує переміщення \
проміжків памʼяті уникаючи блокування основного потоку виконання програмного забезпечення. Цей потік у циклі слідкує за рівнем використання локальної памʼяті та у разі \
якщо він є високим, переміщує проміжки у памʼять віддалених вузлів. Це дозволяє основному потіку виконання не витрачати час на звільнення памʼяті, а одразу перемішувати \
проміжки з памʼяті віддалених вузлів у локальну памʼять. Крім цього, фоновий потік завчасно переміщує сторінки до яких з високою ймовірністю буде виконано доступ у найближчий \
час. Аналогічно, це дозволяє уникнути блокування основного потоку для переміщення даних у локальну памʼять. Якщо припустити що фоновий потік працює максимально швидко та \
обирає проміжки для переміщення максимально правильно, то основний потік не буде блокуватися ніколи, що наблизить швидкодію програмного забезпечення до рівня що є максимально \
близьким до того, коли віддалена памʼять не використовується."),
        paragraph("Легко помітити, що метод вибору проміжків памʼяті для переміщення між локальною памʼяттю та памʼяттю віддалених вузлів має значний вплив на ефективність \
роботи віддаленої памʼяті. Якщо у локальну памʼять переміщуються саме ті проміжки, доступ до яких очікується в першу чергу, а на зберігання у віддалені вузли передаються \
проміжки, які у найближчий час виконання не будуть потрібні, то зменшується кількість блокувань для очіування переміщення потрібних проміжків. Як зазначалось раніше, \
зменшення кількості блокувань покращує швидкодію."),
        paragraph("Таке формулювання вказує на те, що вибір проміжків памʼяті для переміщення є відомою задачею заміщення сторінок (page replacement algorithm). Ця задача \
формулюється як задача керування памʼяттю компʼютера, у якій потрібно обирати яка сторінка буде переміщенна у більш повільну памʼять з більш швидкої замість тієї сторінки \
памʼяті на яку надійшов запит. В схожому вигляді ця задача вирішується, наприклад, у операційних системах: операційній системі потрібно обирати які сторінки памʼяті будуть \
переміщенні у файл підкачки у першу чергу коли потрібно звільнити оперативну памʼять."),
        paragraph("У випадку клієнта віддаленої памʼяті ця задача, як зазначалось раніше, розширюється також тим, що необхідно обирати проміжки памʼяті які навпаки будуть \
завчасно переміщенні з більш повільної памʼяті у більш швидку. Вхідними даними є інформація про проміжки памʼяті до яких відбувався доступ під час поточного та попередніх \
запусків програми. Кожен з проміжків ідентифікується номером, як зазначалось раніше. Надається інформація також про те, які проміжки памʼяті знаходяться у локальній памʼяті, \
а які - у памʼяті віддалених вузлів. Базуючись на цих даних, реалізація алгоритму заміщення сторінок (у коді - ReplacementPolicy) оброблює запити на вибір сторінки для \
переміщення з локальної памʼяті до віддалених вузлів і навпаки. Цей компонент може зберігати внутрішній стан між запитами."),
        paragraph("Для збору статистики про доступ про проміжків памʼяті клієнт віддаленої памʼяті повідомляє алгоритму заміщення сторінок про кожен випадок розіменування \
розумного показчика, за яким зберігаються дані у віддаленій памʼяті. Окремо також повідомляється про кожне переміщення проміжків між типами памʼяті. В залежності від \
реалізації (в цій роботі розглядається декілька), цей компонент може передавати та отримувати статистику в агрегованому вигляді на вузел керування для збереження статистики \
між запусками програми та для агрегування даних між декількома обчислювальними вузлами."),
        paragraph("Найбільш проста реалізація алгоритму заміщення сторінок яка розглядається це заміщення випадковим чином (RandomReplacementPolicy). Це дуже простий \
алгоритм, але він є зручним для встановлення базового рівня для подальшого аналізу. На кожен запит на вибір проміжку для переміщення у віддалену памʼять, цей алгоритм \
випадковим чином обирає проміжок серед усіх які знаходяться у локальній памʼяті. Недоліком цього методу є те, що він обирає проміжки з однаковою ймовірністю, що приводить до \
того, що проміжки які будуть використані найближчим часом під час подальшого виконання програмного забезпечення переміщуються так само, як і проміжки які не будуть потрібні. \
Це призводить до великої кількості випадків коли потрібного проміжку немає в локальній памʼяті, що погіршує швидкодію програмного забезпечення."),
        paragraph("Альтернативою є використання алгоритму заміщення найменш нещодавно використаних (least recently used) проміжків (LeastRecentlyUsedReplacementPolicy). \
Цей алгоритм часто використовується в операційних системах для заміщення сторінок під час використання файлу підкачки, а також у системах кешування - в багатьох випадках \
використання цього алгоритму є виправданим. Так само цей алгоритм може бути виправданим при інтеграції віддаленої памʼяті у програмне забезпечення яке має закономірності \
доступу до памʼяті які роблять цей алгоритм ефективним: програмне забезпечення, у якому до деякої частини даних приходить набагато більше запитів ніж до інших даних. Легко \
помітити неефективність цього алгоритму для програмного забезпечення яке регулярно виконує доступ до проміжків памʼяті у певному порядку. Прикладами програмного забезпечення \
з цього класу може бути програмне забезпечення яке виконує обчислення де значний обсяг даних використовується у певному порядку, наприклад, нейронні мережі чи бази даних які \
виконують сканування таблиці у певному порядку. Для такого програмного забезпечення під час використання віддаленої памʼяті алгоритм заміщення найменш нещодавно використаних \
проміжків буде призводити до низького рівня швидкодії. Це пояснюється тим, що у віддалену памʼять будуть переміщатися саме ті проміжки, доступ до яких буде відбуватися в \
першу чергу під час подальшого виконання програми."),
        paragraph("Очікувано, що поки алгоритм заміщення найменш нещодавно використаних проміжків є неефективним для зазначеного вище класу програмного забезпечення, \"\
зворотній\" алгоритм є ефективним для нього. Заміщення найбільш нещодавно використаних (most recently used) проміжків (MostRecentlyUsedReplacementPolicy) дозволяє отримати \
низький рівень випадків коли потрібного проміжку немає в локальній памʼяті, що має позитивний ефект на швидкодію програмного забезпечення. Так само, недоліком цього алгоритму \
є його неефективність для програмного забезпечення яке має інші закономірності роботи з памʼяттю."),
        paragraph("Оскільки у прикладному програмному забезпеченні закономірності роботи з памʼяттю можуть бути складними та залежити від багатьох факторів, то пошук \
алгоритму заміщення проміжків оптмального для задачі може бути складним для розробників, які використовують віддалену памʼять у своєму програмного забезпеченні. Це \
створює необхідність у алгоритмах заміщення проміжків, які можуть адаптуватися під особливості роботи програмного забезпечення з памʼяттю використовуючи статистику доступу \
до проміжків памʼяті."),
        paragraph("Одним з таких алгоритмів, що розглядаються в цій роботі, є алгоритм заміщення сторінок на основі рекурентної нейронної мережі (recurrent neural network). \
Рекурентні нейронні мережі є розповсюдженим рішенням у задачах де необхідно обробляти послідовності даних. Заміщення сторінок у віддаленій памʼяті є такою задачею. \
Послідовністю в цьому випадку є події доступу до проміжків памʼяті."), // TODO: add more details about RNN
        // tell about policies to evict and pre-fetch spans (and how those use stats collected, heuristics, FSM, ML models, including RNN). explain why grouping objects in spans is effective. explain why it is important to reduce fragmentation and how it can be
        // achieved. tell about compaction.
        // explain what typical performance numbers are in various environments.
        // tell about page placement algorithms
        // explain how different software accesses memory. Tell how "ideal" policy works. Tell why "least recently used" can be a bad policy in some cases.
        // tell how performance is analyzed using tracing.
    ])
}
