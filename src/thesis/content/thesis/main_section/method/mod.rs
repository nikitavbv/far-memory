use crate::thesis::engine::{Block, section_header, subsection_header, paragraph, SubsectionHeaderBlock};

mod components;
mod integration;

pub fn far_memory_method() -> Block {
    Block::Multiple(vec![
        section_header("Методи та засоби надання віддаленої памʼяті"),
        paragraph("Загалом, принцип роботи віддаленої памʼяті полягає в тому, що програмне забезпечення передає у віддалену памʼять дані для зберігання, а коли до цих даних потрібен доступ, то \
реалізація віддаленої памʼяті за запитом від застосунку переміщує дані з інших більш повільних пристроїв зберігання даних у локальну оперативну памʼять. Після переміщення, програмне \
забезпечення працює з даними так само, як і з будь-якими іншими даними у оперативній памʼяті. Після того, як робота з даними закінчена, вони знов переміщуються на зберігання у інший клас памʼяті. \
Саме це надає можливість зменшити використання оперативної памʼяті вузла та обробляти більше даних ніж обʼєм памʼяті прозоро для програмного забезпечення (тобто без значних змін у код, та те, як \
застосунок працює з даними)."),

        components::components(),
        integration::integration(),

        subsection_header("Забезпечення відмовостійкості"),
        paragraph("Однією з проблем використання віддаленої памʼяті є те, що зберігання даних на інших пристроях має значний негативний вплив на розмір домену збою (failure domain). Тобто якщо \
зазвичай ключову роль у відмовостійкості програмного забезпечення грає деяка ймовірність що вузел обчислення вийде з ладу, то при використанні віддаленої памʼяті додається ймовірність що \
SSD диск (якщо він використовується як бекенд для зберігання даних) чи віддалений вузел (у випадку коли дані розміщуються на віддалених вузлах) вийде з ладу, що приведе до втрати частини даних. \
У випадку віддалених вузлів ризик збою стає значно високим, тому що з великою кількістю вузлів для втрати даних достатьно програмного чи апартного збою на будь-якому з них, тобто ризик втрати даних \
збільшується пропорційно."),
        paragraph("На жаль, не є можливим знизити ризик втрати даних при використанні віддаленої памʼяті до того ж самого рівня, як коли використовується тільки локальна оперативна памʼять. \
Але, можна мінімізувати ризик події коли дані будуть втрачені без можливості відновлення до такого рівня, який буде прийнятним для більшості сценаріїв використання на практиці."),
        paragraph("Найбільш простим методом забезпечення відмовостійкості є для кожного проміжку памʼяті (span) що було переміщено у памʼять віддаленого вузла, зберігати ще одну копію даних на \
локальному SSD диску. Недоліком цього підходу є високий рівень затримки у разі відмови вузла зберігання: читання даних з диску є набагато більш повільним ніж отримання даних по мережі, тому у разі \
відмови це негативно вплине на швидкодію програмного забезпечення, у яке інтегрована віддалена памʼять. Крім цього, це призводить до використання додаткового ресурсу - дискового простору, в результаті \
баланс між більш ефективним використанням оперативної памʼяті та додатковим використанням диску буде мати негативний вплив на розмір простору можливого застосування віддаленої памʼяті на практиці. \
Використання дискового простору можна зменшити використовуючи алогоритми стискання даних, такі як zstd, але це зробить систему більш повільною (найбільш швидкі алгоритми стискання даних обробляють \
на рівні сотень мегабайт у секунду) та збільшить використання ресурсів процесору, що не є бажаним при використанні віддаленої памʼяті..."),
        // tell about replication to remote nodes and local SSDs and erasure coding. Tell how exactly data will be restored and deleted. Explain that failure domain becomes larger when far memory is used.
        // tell about healthchecks.

        subsection_header("Забезпечення швидкодії віддаленої памʼяті"),
        // tell about optimizing network requests (why TCP (also, why nodelay is used and duplex) is used and not UDP, or http or some kind of existing RPC implementation).
        // tell about reasoning behind partial swap in/swap out. tell why compression is not used. tell why copies should be avoided. tell a bit about size classes.
        // tell about background swap in and swap out threads and how synchronization should be performed.
        // explain what is the key in minimizing latency (keeping all the needed memory locally and moving it quickly) - like explained in the docs.
        // tell that only 3 out of 5 data shards are needed to minimize latency when restoring data.
        // tell about policies to evict and pre-fetch spans (and how those use stats collected, heuristics, FSM, ML models, including RNN). explain why grouping objects in spans is effective. explain why it is important to reduce fragmentation and how it can be
        // achieved. tell about compaction.
        // explain what typical performance numbers are in various environments.
        // tell about page placement algorithms
        // explain how different software accesses memory. Tell how "ideal" policy works. Tell why "least recently used" can be a bad policy in some cases.

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        // general conclusions

        // in third section explain that a demo app was implemented to measure how well everything works and the hardware of the test environment.
    ])
}
